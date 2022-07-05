use std::path::Path;

use litrs::StringLit;
use oas3::OpenApiV3Spec;
use proc_macro::TokenStream;
use quote::quote;
mod generate_routing;
mod generate_trait;
mod generate_types;
pub(crate) mod utils;

fn extract_path(input: TokenStream) -> Result<String, String> {
  let mut items = input.into_iter().collect::<Vec<_>>();
  if items.len() != 1 {
    Err("Expected only one argument".into())
  } else {
    let token = items.remove(0);
    match StringLit::try_from(token) {
      Ok(token) => Ok(token.value().to_owned()),
      Err(err) => Err(err.to_compile_error().to_string()),
    }
  }
}

fn read_spec<P>(path: P) -> Result<OpenApiV3Spec, String>
where
  P: AsRef<Path>,
{
  let root = path.as_ref().parent();
  let file = path.as_ref().file_name();
  if let (Some(root), Some(file)) = (root, file) {
    oas3::from_path_dir(root.display().to_string(), file.to_str().unwrap()).map_err(|err| {
      format!(
        "\"Cannot parse file: {} Error: {err}\"",
        path.as_ref().display()
      )
    })
  } else {
    Err("Cannot get filename and path correctly".into())
  }
}

#[proc_macro]
pub fn types(input: TokenStream) -> TokenStream {
  let schema = extract_path(input).and_then(read_spec);

  let q = match schema {
    Ok(openapi) => {
      let router = generate_routing::run(&openapi);
      let api_trait = generate_trait::run(&openapi);
      let types = generate_types::run(&openapi);
      quote! {
        pub mod types {
          #types
        }
        pub mod traits {
          #api_trait
        }
        pub mod router {
          #router
        }
      }
    }
    Err(err) => {
      quote! {
        compile_error!(#err);
      }
    }
  };

  TokenStream::from(q)
}
