CREATE TABLE notifications
(
    id            INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id       INTEGER NOT NULL,
    content       TEXT    NOT NULL,
    category      TEXT    NOT NULL,
    was_it_viewed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at    DATE    NOT NULL DEFAULT CURRENT_DATE,
    updated_at    DATE
);