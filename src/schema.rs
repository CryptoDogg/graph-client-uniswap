table! {
    swaps (id) {
        id -> Int4,
        time -> Timestamp,
        sold -> Bool,
        symbol -> Text,
        count -> Int4,
    }
}

table! {
    tokens (id) {
        id -> Int4,
        address -> Varchar,
        symbol -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    swaps,
    tokens,
);
