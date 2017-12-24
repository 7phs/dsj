CREATE TABLE IF NOT EXISTS vectors (
    word_id INTEGER NOT NULL,
    position INTEGER NOT NULL,
    point REAL NOT NULL,
    PRIMARY KEY (word_id, position),
    FOREIGN KEY (word_id) REFERENCES words(id)
);