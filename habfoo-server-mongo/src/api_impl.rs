use async_trait::async_trait;
use habfoo_api_generated::models;
use habfoo_api_generated::{
  Api, CreateFoodItemResponse, GetFoodItemResponse, SearchFoodItemResponse,
};
use log::info;
use swagger::{ApiError, Has, XSpanIdString};

#[derive(Clone)]
pub struct HabFooApi {
  database_client: mongodb::Client,
}

impl HabFooApi {
  pub async fn new(mongodb_uri: &str) -> Self {
    let database_client = mongodb::Client::with_uri_str(mongodb_uri)
      .await
      .unwrap_or_else(|_| panic!("Cannot create mongodb client with addr {}", mongodb_uri));
    info!("Good");
    HabFooApi { database_client }
  }
}

#[async_trait]
impl<C> Api<C> for HabFooApi
where
  C: Has<XSpanIdString> + Send + Sync,
{
  async fn create_food_item(
    &self,
    food_item_inner: models::FoodItemInner,
    context: &C,
  ) -> Result<CreateFoodItemResponse, ApiError> {
    info!(
      "create_food_item({:?}) - X-Span-ID: {:?}",
      food_item_inner,
      context.get().0.clone()
    );
    Err(ApiError("Generic failure".into()))
  }

  async fn get_food_item(
    &self,
    item_id: String,
    context: &C,
  ) -> Result<GetFoodItemResponse, ApiError> {
    info!(
      "get_food_item({:?}) - X-Span-ID: {:?}",
      item_id,
      context.get().0.clone()
    );
    Err(ApiError("Generic failure".into()))
  }

  async fn search_food_item(
    &self,
    search_input: Option<models::SearchInput>,
    context: &C,
  ) -> Result<SearchFoodItemResponse, ApiError> {
    info!(
      "search_food_item({:?}) - X-Span-ID: {:?}",
      search_input,
      context.get().0.clone()
    );
    Err(ApiError("Generic failure".into()))
  }
}
