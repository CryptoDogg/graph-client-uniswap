use crate::schema::tokens;
use crate::schema::swaps;

#[derive(Queryable)]
pub struct Token {
    pub id: i32,
    pub address: String,
    pub symbol: String,
}

#[derive(Queryable)]
pub struct Swap {
    pub id: i32,
    pub time: chrono::NaiveDateTime,
    pub sold: bool,
    pub symbol: String,
    pub count: i32,
}

#[derive(Insertable)]
#[table_name = "tokens"]
pub struct NewToken<'a> {
    pub address: &'a str,
    pub symbol: &'a str,
}

#[derive(Insertable)]
#[table_name = "swaps"]
pub struct NewSwap<'a> {
    pub time: &'a chrono::NaiveDateTime,
    pub sold: &'a bool,
    pub symbol: &'a str,
    pub count: &'a i32,
}