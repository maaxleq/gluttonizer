use std::{fs::read_to_string};

use crate::{entity::Restaurants, error::ProgramError};

pub trait DataProvide {
    fn get_data(&self) -> Result<Restaurants, ProgramError>;
}

pub struct JsonProvider {
    pub path: String,
}

impl JsonProvider {
    pub fn new(path: String) -> JsonProvider {
        JsonProvider { path }
    }
}

impl DataProvide for JsonProvider {
    fn get_data(&self) -> Result<Restaurants, ProgramError> {
        Ok(serde_json::from_str(&read_to_string(self.path.clone())?)?)
    }
}
