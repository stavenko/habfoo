use oas3::spec::SchemaType;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;


fn resolve_schema_type(schema: &oas3::Schema) -> TokenStream {
  match schema.schema_type {
    Some(SchemaType::Object) => {
      let ident = Ident::new(schema.title.as_ref().unwrap(), Span::call_site());
      quote!(#ident)

    },
    Some(SchemaType::String) => {
      if let Some(format) = schema.format.as_ref() {
        match format.as_str() {
          "time" => quote!(std::time::SystemTime),
          "date" => quote!(std::time::SystemTime),
          "date-time" =>  quote!(std::time::SystemTime),
          _ => quote!(String)
        }

      }else {
        quote!(String)
      }
    }
    Some(SchemaType::Number) => {
      if let Some(format) = schema.format.as_ref() {
        match format.as_str() {
          "int64" => quote!(i64),
          "double"=> quote!(f64),
          _ => quote!(f64),
            
        }
      }else {
        quote!(f64)
      }
    }
    Some(SchemaType::Boolean)  => quote!(bool),
    Some(SchemaType::Integer) => quote!(i64),
    Some(SchemaType::Array) => quote!(Vec<String>),
    None => {
      unreachable!();
    }
  }
}

fn extract_struct_fields(schema: &oas3::Schema, spec: &oas3::Spec) -> (Vec<TokenStream>, Vec<Option<TokenStream>>) {
  schema.properties.iter()
    .map(|(name, field_type)| {
      let name = Ident::new(&name, Span::call_site());
      let field_schema = field_type.resolve(spec).ok();
      let type_name = resolve_schema_type(field_schema.as_ref().unwrap());

      (quote!(#name: #type_name), schema_to_type(field_schema.as_ref().unwrap(), spec))
    }).unzip()
}

fn schema_to_type(schema: &oas3::Schema, spec: &oas3::Spec) -> Option<TokenStream> {
  if let Some(SchemaType::Object) = &schema.schema_type {

    let type_name = Ident::new(schema.title.as_ref().unwrap(), Span::call_site());
    let (struct_fields, struct_types) = extract_struct_fields(&schema, spec); 
    Some(quote!(

    #(#struct_types)*
    pub struct #type_name {
      #(#struct_fields), *

    }))
  } else {
    None
  }
}

fn collect_schemas_from_params(spec: &oas3::Spec) -> Vec<TokenStream> {
  spec.operations().flat_map(|(_path, _method, op)| {
    op.parameters.iter().filter_map(|param| {
      param
        .resolve(spec)
        .ok()
        .and_then(|param| param.schema.and_then(|sc| sc.resolve(spec).ok()))
        .and_then(|sc| schema_to_type(&sc, spec))
    })
  })
  .collect()

}

pub fn run(spec: &oas3::Spec) -> TokenStream {
  let param_types = collect_schemas_from_params(spec);
  quote! {
    #(#param_types)*
  }
}
