CREATE TYPE user_role      AS ENUM ('ADMIN', 'SUPPORT', 'DEVELOPER');
CREATE TYPE task_category  AS ENUM ('BUG', 'TASK', 'RESEARCH');

CREATE TABLE users (
   id          UUID PRIMARY KEY,
   first_name  TEXT NOT NULL,
   last_name   TEXT NOT NULL,
   headline    TEXT,
   role        user_role NOT NULL
);

CREATE TABLE tasks (
   id          SERIAL PRIMARY KEY,
   title       TEXT NOT NULL,
   category    task_category NOT NULL,
   description TEXT,
   creator_id  UUID REFERENCES users(id) NOT NULL,
   assigned_to UUID REFERENCES users(id)
);
