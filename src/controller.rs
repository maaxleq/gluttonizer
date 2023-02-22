use crate::{template::HomeTemplate, error::RequestError, service::{StandardService, Service}};

pub async fn home() -> Result<HomeTemplate, RequestError> {
    Ok(HomeTemplate {
        restaurants: StandardService::get_all_restaurants()?
    })
}
