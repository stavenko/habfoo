use habfoo_api_generated::server::MakeService;
use swagger::EmptyContext;
use crate::api_impl::HabFooApi;

pub async fn create(addr: &str, mongodb_addr: &str) {
  let addr = addr.parse().expect("Failed to bind address");

  let server = HabFooApi::new(mongodb_addr).await;
  let service = MakeService::new(server);

  let service = habfoo_api_generated::server::context::MakeAddContext::<_, EmptyContext> :: new(service);

  hyper::server::Server::bind(&addr).serve(service).await.unwrap()
}


