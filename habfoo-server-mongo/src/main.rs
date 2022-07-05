// mod api_impl;
// mod server;
//
use openapi_types_generator::types;
use routerify;

types!("habfoo-api/api/root.yaml");

async fn run() {
  /*
  env_logger::init();
  let mongodb_user = std::env::var("MONGODB_USER").expect("MONGODB_USER is a mandatory var");
  let mongodb_db = std::env::var("MONGODB_DB").expect("MONGODB_DB is a mandatory var");
  let mongodb_password =
    std::env::var("MONGODB_PASSWD").expect("MONGODB_PASSWD is a mandatory var");
  let mongodb_host = std::env::var("MONGODB_HOST").expect("MONGODB_HOST is a mandatory var");
  let addr = "127.0.0.1:8080";
  let mongodb_addr_url =
    format!("mongodb://{mongodb_user}:{mongodb_password}@{mongodb_host}:27019/habfoo");

  let schema = oas3::from_path_dir("habfoo-api/api", "root.yaml").unwrap();
  let search_food_itam = schema.paths.get("/search-food-item").unwrap();
  let parameter = &search_food_itam.get.as_ref().unwrap().parameters[0].resolve(&schema).unwrap();

  println!("{:#?}", parameter.schema.as_ref().unwrap().resolve(&schema));
  */

  // server::create(addr, &mongodb_addr_url).await;
}

#[tokio::main]
async fn main() {
  run().await;
}
