#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{NewToken, Token};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_token(conn: &PgConnection, address: &str, symbol: &str) -> Token {
    use schema::tokens;

    let new_token = NewToken { address, symbol };

    diesel::insert_into(tokens::table)
        .values(&new_token)
        .get_result(conn)
        .expect("Error saving new token")
}