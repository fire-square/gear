CREATE TABLE users (
  id uuid PRIMARY KEY,
  username VARCHAR(16) NOT NULL UNIQUE
)