use axum::Form;

use crate::{template::{HomeTemplate, ResultsTemplate, RestaurantTemplate}, error::RequestError, service::{StandardService, Service}, entity::GluttonRequest};

pub async fn home() -> Result<HomeTemplate, RequestError> {
    Ok(HomeTemplate {
        restaurants: StandardService::get_all_restaurants()?
    })
}

pub async fn gluttonize(Form(request): Form<GluttonRequest>) -> Result<ResultsTemplate, RequestError> {
    Ok(ResultsTemplate { results: StandardService::find_and_pick_restaurants(request)? })
}

pub async fn restaurants() -> Result<RestaurantTemplate, RequestError> {
    Ok(RestaurantTemplate {
        restaurants: StandardService::get_all_restaurants()?
    })
}
