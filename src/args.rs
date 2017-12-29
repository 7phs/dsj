use clap::{Arg, App};
use wordvector::VectorFile;

struct Argument<'a> {
    name: &'a str,
    short: &'a str,
    long: &'a str,
    help: &'a str,
    file_path: fn(String) -> VectorFile,
}

pub struct Args<'a> {
    app: App<'a, 'a>,
    file_path: Option<Vec<VectorFile>>,
    database_uri: Option<String>,
}

impl<'a> Default for Args<'a> {
    fn default() -> Args<'a> {
        let app = App::new("Word Vector To DB")
            .version("0.1")
            .author("Alexey Piyanin")
            .about("Convert a word vector file (fasttext, glove, word2vec, gensim) to DB")
            .arg(Arg::with_name("db_uri")
                .short("db")
                .long("db")
                .takes_value(true)
                .default_value("wordvector.db")
                .help("path to result database"));

        let arguments = [
            Argument {
                name: "fasttext",
                short: "ft",
                long: "fasttext",
                help: "fasttext word vector text file (*.vec)",
                file_path: VectorFile::FastText,
            },
            Argument {
                name: "glove",
                short: "gl",
                long: "glove",
                help: "glove word vector text file (*.txt)",
                file_path: VectorFile::Glove,
            },
            Argument {
                name: "word2vec",
                short: "w2v",
                long: "word2vec",
                help: "word2vec word vector text file (*.txt)",
                file_path: VectorFile::Word2Vec,
            },
            Argument {
                name: "word2vec/gensim",
                short: "t",
                long: "gensim",
                help: "word2vec word vector text file with gensim format (*.tsv)",
                file_path: VectorFile::Gensim,
            },
        ];

        let app = arguments.iter().fold(
            app,
            |arg, argument| arg.arg(
                Arg::with_name(argument.name)
                    .short(argument.short)
                    .long(argument.long)
                    .takes_value(true)
                    .help(argument.help)
            ),
        );

        let args = app.clone().get_matches();

        let database_uri = match args.value_of("db_uri") {
            Some(database_uri) => Some(database_uri.to_string()),
            None => None,
        };
        let file_path: Vec<VectorFile> = arguments.iter().filter_map(|arg_info| {
            let file_path = args.value_of(arg_info.name)?;

            Some((arg_info.file_path)(file_path.to_string()))
        }).collect();

        Args {
            app,
            file_path: if file_path.len() > 0 {
                Some(file_path)
            } else {
                None
            },
            database_uri,
        }
    }
}

impl<'a> Args<'a> {
    pub fn is_incomplete(&self) -> bool {
        if let None = self.file_path {
            return true;
        }

        if let None = self.database_uri {
            return true;
        }

        false
    }

    pub fn print_help(&mut self) {
        self.app.print_help();
        println!();
    }

    pub fn file_path(&self) -> Option<&[VectorFile]> {
        match self.file_path {
            Some(ref file_path) => Some(&file_path),
            None => None,
        }
    }

    pub fn database_uri(&self) -> Option<&str> {
        match self.database_uri {
            Some(ref database_uri) => Some(&database_uri),
            None => None,
        }
    }
}