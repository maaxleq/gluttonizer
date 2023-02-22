use std::sync::{RwLock, Arc};

use lazy_static::lazy_static;

use crate::{entity::{Restaurants, GluttonRequest, GluttonResponse}, error::{RequestError, ProgramError}, picker::{Pick, StandardPicker}, data_provider::{JsonProvider, DataProvide}};

pub trait Repository {
    type Picker: Pick;

    fn find_all() -> Result<Restaurants, RequestError>;
    fn find_and_pick(request: GluttonRequest) -> Result<GluttonResponse, RequestError> {
        Ok(Self::Picker::pick(Self::find_all()?, request))
    }
}

lazy_static! {
    static ref RESTAURANTS: Arc<RwLock<Option<Restaurants>>> = Arc::new(RwLock::new(None));
}

pub struct StandardRepository {}

impl StandardRepository {
    pub fn init() -> Result<(), ProgramError> {
        let provider = JsonProvider::new(String::from("data/restaurants.json"));
        *RESTAURANTS.write()? = Some(provider.get_data()?);

        Ok(())
    }
}

impl Repository for StandardRepository {
    type Picker = StandardPicker;

    fn find_all() -> Result<Restaurants, RequestError> {
        match RESTAURANTS.read() {
            Ok(lock) => match &*lock {
                Some(restaurants) => Ok(restaurants.clone()),
                None => Err(RequestError::service_unavailable().with_msg("Repository is not initialized"))
            },
            Err(_) => Err(RequestError::service_unavailable().with_msg("Could not lock repository"))
        }
    }
}
