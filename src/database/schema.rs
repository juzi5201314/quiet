table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
        create_time -> BigInt,
        comments -> Nullable<Integer>,
    }
}
