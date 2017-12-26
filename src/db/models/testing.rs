use rand;
use rand::Rng;
use diesel::sqlite::SqliteConnection;
use db::sqlite::{establish_connection, run_migration};

fn prepare_connection() -> Result<SqliteConnection, String> {
    let connection = establish_connection(":memory:")?;

    run_migration(&connection)?;

    Ok(connection)
}

fn random_f(probability_zero: f32, min: f32, max: f32) -> Box<Fn() -> f32> {
    Box::new(move || {
        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < probability_zero {
            0.0f32
        } else {
            rng.gen_range::<f32>(min, max)
        }
    })
}


fn random_vector(n: usize) -> Vec<f32> {
    let rnd = random_f(0.3f32, -100.0f32, 100.0f32);

    vec![0f32; n].into_iter().map(|_| rnd()).collect()
}

#[test]
fn test_kinds() {
    use super::kind::{create_kind, get_kind, kinds_list};

    let connection = match prepare_connection() {
        Ok(connection) => connection,
        Err(err) => {
            assert!(false, "failed to prepare connection with {:?}", err);
            return;
        }
    };

    let kinds = vec!["намело", "сугробы", "крыльца"];

    for &kind in kinds.iter() {
        match create_kind(&connection, kind) {
            Some(k) => assert_eq!(k.name, kind, "check creation"),
            None => assert!(false, "failed to create kind - '{}'", kind),
        }
    }

    assert!(kinds.iter().all(|&kind| {
        match get_kind(&connection, kind) {
            Some(rec) => rec.id > 0,
            None => false,
        }
    }), "check exists and id");

    match kinds_list(&connection) {
        Some(list) => assert_eq!(
            &list.iter().map(|rec|
                rec.name.to_string()
            ).collect::<Vec<_>>(),
            &kinds,
            "check list content"
        ),
        None => assert!(false, "failed to get stored kind values"),
    }
}

#[test]
fn test_words() {
    use super::word::{create_word, get_word, words_list};

    let connection = match prepare_connection() {
        Ok(connection) => connection,
        Err(err) => {
            assert!(false, "failed to prepare connection with {:?}", err);
            return;
        }
    };

    let words = vec!["намело", "сугробы", "крыльца"];

    for &word in words.iter() {
        match create_word(&connection, word) {
            Some(w) => assert_eq!(w.word, word, "check creation"),
            None => assert!(false, "failed to create word - '{}'", word),
        }
    }

    assert!(words.iter().all(|&word| {
        match get_word(&connection, word) {
            Some(rec) => rec.id > 0,
            None => false,
        }
    }), "check exists and id");

    match words_list(&connection) {
        Some(list) => assert_eq!(
            &list.iter().map(|rec|
                rec.word.to_string()
            ).collect::<Vec<_>>(),
            &words,
            "check list content"
        ),
        None => assert!(false, "failed to get stored words values"),
    }
}

#[test]
fn test_vectors() {
    use std::collections::BTreeMap;
    use super::word::{Word, create_word};
    use super::kind::{Kind, create_kind};
    use super::vector::{Vector, add_vectors, word_2_vector};

    let connection = match prepare_connection() {
        Ok(connection) => connection,
        Err(err) => {
            assert!(false, "failed to prepare connection with {:?}", err);
            return;
        }
    };

    let kinds = vec!["glove", "fasttext", "word2vec"];
    let words = vec!["намело", "сугробы", "у", "нашего", "крыльца"];

    let stored_kinds = kinds.iter().filter_map(|&kind| {
        create_kind(&connection, kind)
    }).collect::<Vec<Kind>>();

    let stored_words = words.iter().filter_map(|&word| {
        create_word(&connection, word)
    }).collect::<Vec<Word>>();

    let mut data: BTreeMap<(String, String), Vec<f32>> = BTreeMap::new();
    let vector_dim: usize = 30;

    for ref kind in stored_kinds.iter() {
        for ref word in stored_words.iter() {
            let key = (kind.name.to_string(), word.word.to_string());

            let vec = random_vector(vector_dim);

            if add_vectors(&connection, &Vector::from_vec(&word, &kind, &vec))==0 {
                assert!(false, "failed to add vectors for {:?} under {:?}", word, kind);
            } else {
                data.insert(key, vec);
            }
        }
    }

    for ref kind in stored_kinds.iter() {
        for ref word in stored_words.iter() {
            let exist_vec = match word_2_vector(&connection, &word, &kind) {
                Some(vec) => vec,
                None => {
                    assert!(false, "failed to get a vector for {:?} under {:?}", word, kind);
                    continue;
                }
            };

            let key = (kind.name.to_string(), word.word.to_string());

            if let Some(expected_vec) = data.get(&key) {
                assert_eq!(&exist_vec, expected_vec, "check vec");
            } else {
                assert!(false, "data doesn't found in a test cache");
            }
        }
    }
}