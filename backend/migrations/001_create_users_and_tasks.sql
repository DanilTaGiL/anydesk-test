CREATE TABLE users (
   id          UUID PRIMARY KEY,
   first_name  TEXT NOT NULL,
   last_name   TEXT NOT NULL,
   headline    TEXT,
   role        TEXT NOT NULL
);

CREATE TABLE tasks (
   id          SERIAL PRIMARY KEY,
   title       TEXT NOT NULL,
   category    TEXT NOT NULL,
   description TEXT,
   creator_id  UUID REFERENCES users(id),
   assigned_to UUID REFERENCES users(id)
);
