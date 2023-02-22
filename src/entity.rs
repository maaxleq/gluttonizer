use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Restaurant {
    pub name: String,
    pub max_people: u32,
    pub total_travel_time_minutes: u32,
}

pub type Restaurants = Vec<Restaurant>;

#[derive(Clone)]
pub struct GluttonRequest {
    pub exclude: Vec<String>,
    pub people_count: u32,
    pub available_time_minutes: u32,
    pub minimal_eat_time_minutes: u32
}

pub struct GluttonSuggestion {
    pub restaurant_name: String,
    pub time_to_eat_minutes: u32
}

pub type GluttonResponse = Vec<GluttonSuggestion>;
