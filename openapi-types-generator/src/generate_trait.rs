use std::{
  collections::{HashMap, HashSet},
  fmt::format,
};

use convert_case::{Case, Casing};
use oas3::spec::{Operation, Parameter};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

fn collect_params(schema: &oas3::Spec, operation: &Operation) -> Vec<TokenStream> {
  operation
    .parameters
    .iter()
    .map(|p| p.resolve(schema).unwrap_or_else(|_| Parameter::default()))
    .map(|param_object| {
      let name = Ident::new(&param_object.name.to_case(Case::Snake), Span::call_site());
      let type_name: String = param_object
        .schema
        .and_then(|sc| sc.resolve(schema).ok())
        .and_then(|sc| {
          sc.title
            .or_else(|| sc.schema_type.map(|tp| format!("{:?}", tp)))
        })
        .unwrap_or_else(|| "None".into());
      let type_name = Ident::new(&type_name.to_case(Case::Pascal), Span::call_site());
      quote!( #name: #type_name)
    })
    .collect::<Vec<_>>()
}

fn collect_response_types(
  schema: &oas3::Spec,
  operation: &Operation,
) -> HashMap<String, HashMap<String, String>> {
  operation
    .responses
    .iter()
    .flat_map(|(code, req_body)| {
      req_body.resolve(&schema).map(|response| {
        (
          code.to_owned(),
          response
            .content
            .iter()
            .map(|(content_type, media_type)| {
              let my_type = media_type
                .schema
                .as_ref()
                .and_then(|sc| sc.resolve(schema).ok())
                .and_then(|sc| sc.title.or(sc.schema_type.map(|tp| format!("{:?}", tp))))
                .unwrap_or("NotFoundTypeName".into());
              (content_type.to_owned(), my_type)
            })
            .collect(),
        )
      })
    })
    .collect()
}

fn collect_body_params(schema: &oas3::Spec, operation: &Operation) -> Vec<TokenStream> {
  operation
    .request_body
    .as_ref()
    .and_then(|rb| rb.resolve(schema).ok())
    .map(|req_body| {
      req_body
        .content
        .iter()
        .flat_map(|(_type, media)| {
          media
            .schema
            .as_ref()
            .and_then(|sc| sc.resolve(schema).ok())
            .and_then(|sc| sc.title)
            .map(|title| {
              (
                Ident::new(&title.to_case(Case::Snake), Span::call_site()),
                Ident::new(&title.to_case(Case::Pascal), Span::call_site()),
              )
            })
            .map(|(var, tpe)| quote!(#var: #tpe))
        })
        .collect()
    })
    .unwrap_or_else(Vec::new)
}

fn code_to_name(code: &str) -> String {
  format!("Result_{code}")
}

enum SomeType {
  EnumOfCodes(Vec<TokenStream>),
  SingleType(Ident),
  NoReturn,
}

fn create_media_types_result(method_name: &str, result_types: &HashSet<String>) -> Option<Ident> {
  if result_types.len() == 1 {
    Some(
      result_types
        .iter()
        .find(|_| true)
        .map(|type_name| Ident::new(type_name, Span::call_site()))
        .expect("Must find single item in map of size 1"),
    )
  } else if result_types.is_empty() {
    None
  } else {
    let method_name = method_name.to_case(Case::Pascal);
    Some(Ident::new(
      &format!("{method_name}Result"),
      Span::call_site(),
    ))
  }
}

fn create_enum_variants(
  method_name: &str,
  code_conten_type: &HashMap<String, HashSet<String>>,
) -> SomeType {
  if code_conten_type.len() > 1 {
    SomeType::EnumOfCodes(
      code_conten_type
        .iter()
        .map(|(code, result_types)| {
          let variant_name = Ident::new(&code_to_name(code), Span::call_site());
          if let Some(inner_variant_type) = create_media_types_result(method_name, result_types) {
            quote!(#variant_name (#inner_variant_type))
          } else {
            quote!(#variant_name)
          }
        })
        .collect(),
    )
  } else if let Some(ident) = code_conten_type
    .iter()
    .find(|_| true)
    .and_then(|(_, result_types)| create_media_types_result(method_name, result_types))
  {
    SomeType::SingleType(ident)
  } else {
    SomeType::NoReturn
  }
}

fn build_result_type(
  method_name: &str,
  code_conten_type: HashMap<String, HashMap<String, String>>,
) -> (TokenStream, Option<TokenStream>) {
  let type_name = format!("{method_name}_result_type").to_case(Case::Pascal);

  let type_name_ident = Ident::new(&type_name, Span::call_site());
  let code_conten_type_set: HashMap<String, HashSet<String>> = code_conten_type
    .iter()
    .map(|(code, map)| {
      (
        code.to_owned(),
        map
          .iter()
          .map(|(_ct, type_name)| type_name.to_owned())
          .collect::<HashSet<_>>(),
      )
    })
    .collect();

  match create_enum_variants(method_name, &code_conten_type_set) {
    SomeType::EnumOfCodes(variants) => (
      quote!( #type_name_ident ),
      Some(quote!(
          pub enum #type_name_ident {
            #(#variants),*

          }
      )),
    ),
    SomeType::SingleType(result_type) => (quote!(#result_type), None),

    SomeType::NoReturn => (quote!(()), None),
  }
}

pub fn run(schema: &oas3::Spec) -> TokenStream {
  let title = Ident::new(&schema.info.title, Span::call_site());
  let (methods,result_types)  = schema
    .paths
    .iter()
    .flat_map(|(_path, item)| {
      item
        .methods()
        .into_iter()
        .map(|(_method, op)| {
          let method_name = op
            .operation_id
            .as_ref()
            .expect("Operation id is not provided for {path} method: {method}");
          let method_name_ident = Ident::new(&method_name.to_case(Case::Snake), Span::call_site());
          let params = collect_params(schema, op);
          let body_parames = collect_body_params(schema, op);
          let (result, _result_type) = build_result_type(method_name, collect_response_types(schema, op));
          (quote! {async fn #method_name_ident(&self, #(#params),* #(#body_parames),*) -> #result;},
          _result_type)
        })
        .collect::<Vec<_>>()
    })
    .unzip::<_, _, Vec<_>, Vec<_>>();

  quote! {

    use async_trait::async_trait;

    #(#result_types) *

    #[async_trait]
    pub trait #title {
      #(#methods)*
    }
  }
}
