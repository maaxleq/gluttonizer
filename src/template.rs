use askama::Template;

use crate::entity::Restaurants;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub restaurants: Restaurants
}
