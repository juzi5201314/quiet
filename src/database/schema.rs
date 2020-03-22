table! {
    posts (id) {
        id -> Text,
        title -> Text,
        content -> Text,
        create_time -> BigInt,
        comments -> Nullable<Integer>,
    }
}
