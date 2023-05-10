use std::collections::HashMap;

use serde::{Deserialize};

use crate::error::RequestError;

#[derive(Deserialize, Clone)]
pub struct Restaurant {
    pub name: String,
    pub max_people: u32,
    pub total_travel_time_minutes: u32,
}

pub type Restaurants = Vec<Restaurant>;

#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "HashMap<String, String>")]
pub struct GluttonRequest {
    pub exclude: Vec<String>,
    pub people_count: u32,
    pub available_time_minutes: u32,
    pub minimal_eat_time_minutes: u32,
}

impl TryFrom<HashMap<String, String>> for GluttonRequest {
    type Error = RequestError;

    fn try_from(map: HashMap<String, String>) -> Result<Self, Self::Error> {
        let people_count = match map.get("people_count") {
            Some(value) => Ok(match value.parse() {
                Ok(value_int) => Ok(value_int),
                Err(_) => Err(RequestError::bad_request()),
            }?),
            None => Err(RequestError::bad_request()),
        }?;

        let available_time_minutes = match map.get("available_time_minutes") {
            Some(value) => Ok(match value.parse() {
                Ok(value_int) => Ok(value_int),
                Err(_) => Err(RequestError::bad_request()),
            }?),
            None => Err(RequestError::bad_request()),
        }?;

        let minimal_eat_time_minutes = match map.get("minimal_eat_time_minutes") {
            Some(value) => Ok(match value.parse() {
                Ok(value_int) => Ok(value_int),
                Err(_) => Err(RequestError::bad_request()),
            }?),
            None => Err(RequestError::bad_request()),
        }?;

        let exclude = map
            .iter()
            .filter(|&(key, _)| key.starts_with("exclude"))
            .map(|(_, val)| val.clone())
            .collect();
        Ok(GluttonRequest {
            exclude,
            people_count,
            available_time_minutes,
            minimal_eat_time_minutes,
        })
    }
}

pub struct GluttonSuggestion {
    pub restaurant_name: String,
    pub time_to_eat_minutes: u32,
}

pub type GluttonResponse = Vec<GluttonSuggestion>;
