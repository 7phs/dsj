table! {
    words {
        id -> Integer,
        word -> VarChar,
    }
}

table! {
    kinds {
        id -> Integer,
        name -> VarChar,
    }
}

table! {
    vectors (word_id, kind_id, position) {
        word_id -> Integer,
        kind_id -> Integer,
        position -> Integer,
        point -> Float,
    }
}