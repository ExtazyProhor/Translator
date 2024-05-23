CREATE TABLE IF NOT EXISTS InputMessages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    string TEXT NOT NULL,
    arraySize INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS ArrayElements (
    messageId INTEGER NOT NULL,
    arrayIndex INTEGER NOT NULL,
    intValue INTEGER NOT NULL,
    PRIMARY KEY (messageId, arrayIndex),
    FOREIGN KEY (messageId) REFERENCES InputMessages(id)
);

CREATE TABLE IF NOT EXISTS OutputMessages (
    id INTEGER PRIMARY KEY,
    string TEXT NOT NULL
);