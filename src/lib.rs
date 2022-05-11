#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{NewToken, Token, NewSwap, Swap};

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

pub fn create_swap(conn: &PgConnection, time: &chrono::NaiveDateTime, sold: &bool, symbol: &str, count: &i32) -> Swap {
    use schema::swaps;

    let new_swap = NewSwap { time, sold, symbol, count };

    diesel::insert_into(swaps::table)
        .values(&new_swap)
        .get_result(conn)
        .expect("Error saving new token")
}

#[tokio::main]
pub async fn slack_send(text: String) -> Result<(), reqwest::Error> {
    let slack_webhook = dotenv::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL must be set");
    let response_body = reqwest::Client::new()
        .post(slack_webhook)
        .json(&serde_json::json!({
            "text": text,
        }))
        .send()
        .await?
        .text()
        .await?;

    println!("response body {:#?}", response_body);
    Ok(())
}