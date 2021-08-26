CREATE TABLE users (
        id SERIAL PRIMARY KEY,
        info VARCHAR not null,
        friends INTEGER[],
	email TEXT NOT NULL,
	password TEXT NOT NULL
);
CREATE TABLE users_auth (
  id SERIAL NOT NULL PRIMARY KEY,
  nama TEXT NOT NULL,
  email TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
  password TEXT NOT NULL,
);
