use askama::Template;

use crate::entity::{Restaurants, GluttonResponse};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub restaurants: Restaurants
}

#[derive(Template)]
#[template(path = "results.html")]
pub struct ResultsTemplate {
    pub results: GluttonResponse
}

#[derive(Template)]
#[template(path = "restaurants.html")]
pub struct RestaurantTemplate {
    pub restaurants: Restaurants
}

