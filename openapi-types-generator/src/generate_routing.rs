use std::collections::{BTreeMap, HashSet, HashMap};

use convert_case::{Case, Casing};
use oas3::{
  spec::{MediaType, Operation, Parameter},
  Schema,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

fn function_name(op: &Operation) -> Ident {
  let title = op
    .operation_id
    .as_ref()
    .expect("Operation id is essential")
    .to_case(Case::Snake);
  Ident::new(&title, Span::call_site())
}

fn create_routing_instruction(
  path: &str,
  method: &http::method::Method,
  op: &Operation,
) -> TokenStream {
  let title = function_name(op);
  let method = Ident::new(&method.as_str().to_case(Case::Lower), Span::call_site());

  quote! {
    .#method(#path, #title::<Api>)
  }
}

fn var_from_path(param: &Parameter) -> (TokenStream, TokenStream) {
  let initial_name = param.name.clone();
  let name = param.name.to_case(Case::Snake);
  let name = Ident::new(&name, Span::call_site());
  (
    quote!(let #name = req.param(#initial_name).expect("Param must exist in path");),
    quote!(#name),
  )
}
fn var_from_query(param: &Parameter, spec: &oas3::Spec) -> (TokenStream, TokenStream) {
  let name = param.name.clone();
  let name_snake = name.to_case(Case::Snake);
  let schema = param
    .schema
    .as_ref()
    .and_then(|sc| sc.resolve(spec).ok())
    .expect("Schema shall be exists in parameter in this implementaion");

  let name_snake_id = Ident::new(&name_snake, Span::call_site());
  if schema.title.is_none() {
    (
      quote! {
        let #name_snake_id =  {
          use std::collections::HashMap;
          let query_vars = serde_qs::from_str::<HashMap<String, String>>(req.uri().query().unwrap());
          query_vars.get(#name).cloned().unwrap()
        };
      },
      quote!(#name_snake_id),
    )
  } else {
    let type_name = schema.title.unwrap().to_case(Case::Pascal);
    let type_name_id = Ident::new(&type_name, Span::call_site());
    (
      quote! { let #name_snake_id = serde_qs::from_str::<#type_name_id>(req.uri().query().unwrap());},
      quote!(#name_snake_id),
    )
  }
}
fn var_from_header(param: &Parameter) -> (TokenStream, TokenStream) {
  unimplemented!()
}
fn var_from_cookie(param: &Parameter) -> (TokenStream, TokenStream) {
  unimplemented!()
}

fn var_from_param(param: &Parameter, spec: &oas3::Spec) -> (TokenStream, TokenStream) {
  match param.location.as_str() {
    "query" => var_from_query(param, spec),
    "header" => var_from_header(param),
    "path" => var_from_path(param),
    "cookie" => var_from_cookie(param),
    &_ => unreachable!(),
  }
}

fn get_vars_from_params(op: &Operation, spec: &oas3::Spec) -> Vec<(TokenStream, TokenStream)> {
  op.parameters
    .iter()
    .flat_map(|param| {
      param
        .resolve(spec)
        .map(|param| var_from_param(&param, spec))
    })
    .collect()
}

fn get_parse_instruction(mime: &str, schema: &oas3::Schema) -> TokenStream {
  let type_name = Ident::new(schema.title.as_ref().unwrap(), Span::call_site());
  match mime {
    "application/yaml" => quote!(
      let body = hyper::body::to_bytes(req.body_mut()).await; 
      let body = String::from_utf8(body.to_vec()).unwrap();
      serde_yaml::from_str::<#type_name>(body).unwrap();
      ),
    "application/json" => quote!(
      let body = hyper::body::to_bytes(req.body_mut()).await; 
      let body = String::from_utf8(body.to_vec()).unwrap();
      serde_json::from_str::<#type_name>(body).unwrap();
      ),
      _ => unimplemented!()
  }
}

fn get_vars_from_body(
  content: &BTreeMap<String, MediaType>,
  spec: &oas3::Spec,
) -> Vec<(TokenStream, TokenStream)> {
  let mut total_types = HashMap::new();
  for (mime, media) in content.iter() {
    let schema = media.schema.as_ref().expect("Schema must exists for request_body");
    let schema = schema.resolve(spec).expect("Schema is not resolved");
    let type_name = schema.title.as_ref().cloned().expect("schema title expected");
    if !total_types.contains_key(&type_name) {
      total_types.insert(type_name, vec!((mime, schema.to_owned())));
    }else{
      total_types.get_mut(&type_name).map(|vec| vec.push((mime, schema)));
    }
  }

  let mut types = Vec::new();

  for (type_name, mimes) in total_types.iter() {
    let var_ident = Ident::new(&type_name.to_case(Case::Snake), Span::call_site());
    let type_ident = Ident::new(&type_name.to_case(Case::Pascal), Span::call_site());
     
    let attempts = mimes.iter().map(|(mime, schema)| {
      let parse_instruction = get_parse_instruction(mime, schema);
      quote!(
        .or({ #parse_instruction })
      )
    });


    types.push((quote!(
        let #var_ident = Err(())
          #(#attempts)*
          ;
        ), quote!(#var_ident)
        ));
  }

  types
}

fn get_vars_from_request(op: &Operation, spec: &oas3::Spec) -> Vec<(TokenStream, TokenStream)> {
  op.request_body
    .as_ref()
    .and_then(|rb| rb.resolve(spec).ok())
    .map(|rb| rb.content)
    .map(|content| get_vars_from_body(&content, spec) )
    .unwrap_or(Vec::new())
}

fn get_operation_input(op: &Operation, spec: &oas3::Spec) -> (Vec<TokenStream>, Vec<TokenStream>) {
  let mut from_parames = get_vars_from_params(op, spec);
  let mut from_body = get_vars_from_request(op, spec);
  from_parames.append(&mut from_body);

  from_parames.into_iter().unzip()
}

fn use_operation_result(op: &Operation) -> TokenStream {
  quote!(
  let resutl = hyper::Result::new(hyper::Body::from("Ok"));
  Ok(resutl)
  )
}

fn create_routing_function(op: &Operation, spec: &oas3::Spec) -> TokenStream {
  let title = Ident::new(&spec.info.title, Span::call_site());
  let function_name = function_name(op);
  let (operation_vars, operation_var_names) = get_operation_input(op, spec);
  let use_op = use_operation_result(op);
  quote! {
    async fn #function_name<Api>(req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>> 
      where Api: super::traits::#title
    {
      let api = req.data::<Api>().as_mut();
      #(#operation_vars);*

      let result = api.#function_name(#(#operation_var_names),*).await;

      #use_op

    }
  }
}

fn create_routing_functions(spec: &oas3::Spec) -> (Vec<TokenStream>, Vec<TokenStream>) {
  spec
    .operations()
    .map(|(path, method, op)| {
      (
        create_routing_function(op, spec),
        create_routing_instruction(&path, &method, op),
      )
    })
    .unzip()
}

pub fn run(schema: &oas3::Spec) -> TokenStream {
  let title = Ident::new(&schema.info.title, Span::call_site());
  let (functions, routing_instructions) = create_routing_functions(schema);
  quote! {
    pub fn create_routing_table<Api>(api: Api)
    -> routerify::Router<hyper::Body, Box<dyn std::error::Error + Send + Sync>>
      where Api: super::traits::#title {
      type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
      use routerify::Router;

      /*
      fn some_path(req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>> {
        let api = req.data::<Api>().as_mut();
        api.some_path(req)
      }
      */

      #(#functions)*

      Router::builder()
        .data(api)
        #(#routing_instructions)*
        .build()
        .unwrap()

      }
  }
}
