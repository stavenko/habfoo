mod api_impl;
mod server;

async fn run() {
  env_logger::init();
  log::info!("GGG");
  let addr = "127.0.0.1:8080";

  server::create(addr, "mongodb://127.0.0.1").await;
}

#[tokio::main]
async fn main() {
  run().await;
}
