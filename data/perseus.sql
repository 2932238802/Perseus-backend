
DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id          SERIAL PRIMARY KEY,
    username    VARCHAR(64) UNIQUE NOT NULL,
    password    VARCHAR(255) NOT NULL,        
    created_at  TIMESTAMP DEFAULT NOW()
);