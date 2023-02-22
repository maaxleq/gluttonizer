use std::sync::RwLock;

use crate::entity::Restaurants;

static RESTAURANTS: RwLock<Option<Restaurants>> = RwLock::new(None);

pub fn get() -> Result<Restaurants, ()> {
    todo!();
}
