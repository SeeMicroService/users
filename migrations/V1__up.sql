CREATE TABLE Users
(
    id       uuid PRIMARY KEY ,
    login    text NOT NULL UNIQUE ,
    password text,
    role     text
);

CREATE TABLE Files
(
    user_id  uuid,
    filename TEXT,
    PRIMARY KEY (user_id, filename),
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE
);
