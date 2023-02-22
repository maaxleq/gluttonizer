use crate::{repository::{Repository, StandardRepository}, entity::{Restaurants, GluttonRequest, GluttonResponse}, error::RequestError};

pub trait Service {
    type Repository: Repository;

    fn get_all_restaurants() -> Result<Restaurants, RequestError> {
        Self::Repository::find_all()
    }

    fn find_and_pick_restaurants(request: GluttonRequest) -> Result<GluttonResponse, RequestError> {
        Self::Repository::find_and_pick(request)
    }
}

pub struct StandardService {}

impl Service for StandardService {
    type Repository = StandardRepository;
}
