table! {
    posts (id) {
        id -> Text,
        title -> Text,
        content -> Text,
        create_time -> BigInt,
        update_time -> BigInt,
        comments -> Nullable<Integer>,
    }
}
