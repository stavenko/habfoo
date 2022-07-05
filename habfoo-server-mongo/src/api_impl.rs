use async_trait::async_trait;
use habfoo_api_generated::{
  models::{self, FoodItemInner, Nutrient},
  AddMealResponse, Api, CreateFoodItemResponse, FetchMealsResponse, GetFoodItemResponse,
  RemoveMealResponse, SearchFoodItemResponse,
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

  async fn get_collection<T>(&self, name: &str) -> Option<mongodb::Collection<T>> {
    self
      .database_client
      .default_database()
      .map(|db| db.collection(name))
  }

  async fn get_food_item_collection(&self) -> Option<mongodb::Collection<FoodItemInner>> {
    self.get_collection("foodItems").await
  }
}

#[async_trait]
impl<C> Api<C> for HabFooApi
where
  C: Has<XSpanIdString> + Send + Sync,
{
  async fn add_meal(
    &self,
    add_meal_request: models::AddMealRequest,
    context: &C,
  ) -> Result<AddMealResponse, ApiError> {
    Err(ApiError("Not implemented".into()))
  }

  async fn fetch_meals(
    &self,
    fetch_meals_query: Option<models::FetchMealsRequest>,
    context: &C,
  ) -> Result<FetchMealsResponse, ApiError> {
    Err(ApiError("Not implemented".into()))
  }

  async fn remove_meal(
    &self,
    meal_id: String,
    context: &C,
  ) -> Result<RemoveMealResponse, ApiError> {
    Err(ApiError("Not implemented".into()))
  }

  async fn create_food_item(
    &self,
    food_item_inner: models::FoodItemInner,
    _context: &C,
  ) -> Result<CreateFoodItemResponse, ApiError> {
    if let Some(collection) = self.get_food_item_collection().await {
      collection
        .insert_one(food_item_inner, None)
        .await
        .map(|result| {
          CreateFoodItemResponse::FoodItemCreatedWithNewId(models::CreateFoodItemResult {
            id: result.inserted_id.as_str().unwrap().into(),
          })
        })
        .map_err(|mongo_err| ApiError(format!("{mongo_err}")))
    } else {
      Err(ApiError("Cannot get collection".into()))
    }
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
    Err(ApiError("Not implemented".into()))
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
    Err(ApiError("Not implemented".into()))
  }
}
