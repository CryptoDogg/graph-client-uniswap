use ::reqwest::blocking::Client;
use anyhow::*;
use clap::Parser;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use log::*;
use prettytable::*;

extern crate diesel;
extern crate graphql_query_github_example;

use graphql_query_github_example::*;

#[allow(clippy::upper_case_acronyms)]
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
struct TokensView;

#[derive(Parser)]
#[clap(author, about, version)]
struct Command {
    #[clap(name = "repository")]
    repo: String,
}
fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let variables = tokens_view::Variables {};

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .build()?;

    let response_body = post_graphql::<TokensView, _>(
        &client,
        "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v2
",
        variables,
    )
    .unwrap();

    info!("{:?}", response_body);

    let response_data: tokens_view::ResponseData =
        response_body.data.expect("missing response data");

    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!(b => "id", "symbol"));

    let connection = establish_connection();

    for token in response_data
        .tokens
        .expect("missing tokens")
        .iter()
        .flatten()
    {
        table.add_row(row!(token.id, token.symbol));
        // save token in DB
        let address = &token.id;
        let symbol = &token.symbol;
        create_token(&connection, &address, &symbol);
    }

    table.printstd();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}