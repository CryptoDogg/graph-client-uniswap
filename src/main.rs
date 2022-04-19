use graphql_query_github_example::slack_send;
use ::reqwest::blocking::Client;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use log::*;
// use prettytable::*;

extern crate diesel;
extern crate graphql_query_github_example;

use std::cmp;
use std::collections::{HashMap, HashSet};
use chrono::Local;

// use graphql_query_github_example::*;

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

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .build()?;

    let start_time = Local::now();
    let mut timestamp = start_time.timestamp() - 3600 * 24;

    let mut sold_tokens = HashMap::new();
    let mut bought_tokens = HashMap::new();

    let mut hash_set = HashSet::new();

    let mut index = 0;
    loop {
        let variables = tokens_view::Variables {
            timestamp: timestamp.to_string(),
        };

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

        let hash_set_temp = hash_set.clone();

        for swap in response_data.swaps.expect("missing swaps").iter().flatten() {
            if hash_set.contains(&swap.id) {
                continue;
            }
            hash_set.insert(swap.id.clone());
            timestamp = cmp::max(timestamp, swap.timestamp.parse::<i64>().unwrap());

            let token0_symbol = swap.pair.token0.symbol.clone();
            let token1_symbol = swap.pair.token1.symbol.clone();
            if swap.amount0_in.parse::<f32>().unwrap() > 0.0 {
                let sold_tokens_count = sold_tokens.entry(token0_symbol).or_insert(0);
                *sold_tokens_count += 1;
                let bought_tokens_count = bought_tokens.entry(token1_symbol).or_insert(0);
                *bought_tokens_count += 1;
            } else {
                let sold_tokens_count = sold_tokens.entry(token1_symbol).or_insert(0);
                *sold_tokens_count += 1;
                let bought_tokens_count = bought_tokens.entry(token0_symbol).or_insert(0);
                *bought_tokens_count += 1;
            };
        }

        println!("hash_len {:?}", hash_set.len());
        println!("timestamp {:?}", timestamp);
        println!("index {:?}", index);
        index += 1;
        if hash_set.len() == hash_set_temp.len() {
            break;
        }
    }

    fn hashmap_sort(hashmap: &HashMap<String, usize>) -> Vec<(&String, &usize)> {
        let mut vector: Vec<_> = hashmap.into_iter().collect();
        vector.sort_by(|x, y| y.1.cmp(&x.1));
        vector
    }

    println!("hash_len {:?}", hash_set.len());
    let header = format!("SWAP DATA LAST 24 HOURS AT {}", start_time.format("%Y年%m月%d日 %H:%M:%S"));
    let total_swap = format!("TOTAL_SWAP: {:?}", hash_set.len());
    let text_sold = format!("SOLD_TOP10 {:?}", hashmap_sort(&sold_tokens).get(..10).unwrap());
    let text_bought = format!("BOUGHT_TOP10 {:?}", hashmap_sort(&bought_tokens).get(..10).unwrap());
    println!("{}", text_sold);
    println!("{}", text_bought);
    slack_send( format!("{}{}{}{}{}{}{}", header, "\n", total_swap, "\n", text_sold, "\n", text_bought));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
