use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use std::env;

#[macro_use]
extern crate diesel;
pub mod models;
pub mod schema;

pub fn create_connection() -> Result<PgConnection, failure::Error> {
    dotenv::dotenv().ok();
    Ok(PgConnection::establish(&env::var("DATABASE_URI")?)?)
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
