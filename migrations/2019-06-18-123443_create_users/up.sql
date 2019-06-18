CREATE TABLE users
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    username      CHAR(50) NOT NULL,
    password_hash TEXT     NOT NULL
);
