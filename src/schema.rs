table! {
    words (id) {
        id -> Integer,
        word -> VarChar,
    }
}

table! {
    vectors (word_id, position) {
        word_id -> Integer,
        position -> Integer,
        point -> Float,
    }
}