# graphql-client Uniswap API examples

The repo is referring to the following repos.

[graphql-client example](https://github.com/graphql-rust/graphql-client/tree/main/examples/github).

[diesel example](https://github.com/diesel-rs/diesel/tree/v1.4.4/examples/postgres/getting_started_step_3).

## How to run it

```bash
echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
diesel setup
diesel migration run
cargo run
```
