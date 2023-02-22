use crate::entity::{GluttonRequest, GluttonResponse, GluttonSuggestion, Restaurant};

pub trait Pick {
    fn complies(restaurant: Restaurant, request: GluttonRequest) -> bool;
    fn pick(restaurants: Vec<Restaurant>, request: GluttonRequest) -> GluttonResponse {
        restaurants
            .iter()
            .filter(|restaurant| Self::complies(restaurant.clone().to_owned(), request.clone()))
            .map(|restaurant| GluttonSuggestion {
                restaurant_name: restaurant.name.clone(),
                time_to_eat_minutes: request
                    .available_time_minutes
                    .saturating_sub(restaurant.total_travel_time_minutes),
            })
            .collect()
    }
}

pub struct StandardPicker {}

impl Pick for StandardPicker {
    fn complies(restaurant: Restaurant, request: GluttonRequest) -> bool {
        request.people_count <= restaurant.max_people
            && request.available_time_minutes
                >= request.minimal_eat_time_minutes + restaurant.total_travel_time_minutes
            && !request.exclude.contains(&restaurant.name)
    }
}
