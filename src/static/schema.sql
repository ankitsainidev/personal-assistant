CREATE TABLE notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    message VARCHAR(128) NOT NULL ,
    time INTEGER NOT NULL
);
CREATE TABLE saves (
    key VARCHAR(32) PRIMARY KEY NOT NULL,
    val VARCHAR(128) NOT NULL
);
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description VARCHAR(128) NOT NULL,
    done BOOLEAN NOT NULL DEFAULT 0
);
