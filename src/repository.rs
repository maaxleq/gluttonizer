use std::sync::RwLock;

use crate::{entity::{Restaurants, GluttonRequest, GluttonResponse}, error::RequestError, picker::{Pick, StandardPicker}};

static RESTAURANTS: RwLock<Option<Restaurants>> = RwLock::new(None);

pub trait Repository {
    type Picker: Pick;

    fn find_all() -> Result<Restaurants, RequestError>;
    fn find_and_pick(request: GluttonRequest) -> Result<GluttonResponse, RequestError> {
        Ok(Self::Picker::pick(Self::find_all()?, request))
    }
}

pub struct StandardRepository {}

impl Repository for StandardRepository {
    type Picker = StandardPicker;

    fn find_all() -> Result<Restaurants, RequestError> {
        match RESTAURANTS.read() {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }
}
