use poem::{Endpoint, Route};

use crate::users;

pub fn new() -> impl Endpoint {
    Route::new().nest("api/v1/user", users::new())
}
