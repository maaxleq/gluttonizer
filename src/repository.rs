use std::sync::RwLock;

use axum::http::StatusCode;

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
            Ok(lock) => match lock.into() {
                Some(_) => todo!(),
                None => Err(RequestError { msg: "Repository is not initialized", status: StatusCode::SERVICE_UNAVAILABLE }),
            },
            Err(_) => Err(RequestError { msg: "Could not lock repository", status: StatusCode::SERVICE_UNAVAILABLE })
        }
    }
}
