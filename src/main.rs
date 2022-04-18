use ::reqwest::blocking::Client;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use log::*;
use prettytable::*;

extern crate diesel;
extern crate graphql_query_github_example;

use std::time::Duration;

use graphql_query_github_example::*;

#[allow(clippy::upper_case_acronyms)]
type BigDecimal = String;
type BigInt = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
struct TokensView;

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let variables = tokens_view::Variables {};

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .timeout(Duration::from_secs(180))
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

    // let connection = establish_connection();

    // for token in response_data
    //     .swaps
    //     .expect("missing tokens")
    //     .iter()
    //     .flatten()
    // {
    //     table.add_row(row!(token.id, token.timestamp));
    //     // save token in DB
    //     // let address = &token.id;
    //     // let symbol = &token.symbol;
    //     // create_token(&connection, &address, &symbol);
    // }

    table.printstd();

    #[derive(Debug)]
    struct SwapInfo {
        sold_token: String,
        bought_token: String,
        usd_amount: f32,
    }

    let mut token_list = Vec::new();

    for swap in response_data.swaps.expect("missing swaps").iter().flatten() {
        let usd_amount = swap.amount_usd.parse::<f32>().unwrap();
        let sold_token = swap.pair.token0.symbol.clone();
        let bought_token = swap.pair.token1.symbol.clone();
        let swap_info = if swap.amount0_in.parse::<f32>().unwrap() > 0.0 {
            SwapInfo {
                sold_token,
                bought_token,
                usd_amount,
            }
        } else {
            SwapInfo {
                sold_token: bought_token,
                bought_token: sold_token,
                usd_amount,
            }
        };
        println!("swap_info {:?}", swap_info);
        token_list.push(swap_info);
    }
    println!("token_list {:?}", token_list);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
