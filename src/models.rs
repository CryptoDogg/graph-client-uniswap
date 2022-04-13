use crate::schema::tokens;

#[derive(Queryable)]
pub struct Token {
    pub id: i32,
    pub address: String,
    pub symbol: String,
}

#[derive(Insertable)]
#[table_name = "tokens"]
pub struct NewToken<'a> {
    pub address: &'a str,
    pub symbol: &'a str,
}