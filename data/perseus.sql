DROP TABLE IF EXISTS ai_model;     
DROP TABLE IF EXISTS ai_provider;
DROP TABLE IF EXISTS users;        

CREATE TABLE users (
    id         SERIAL PRIMARY KEY,
    username   VARCHAR(64) UNIQUE NOT NULL,
    password   VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE ai_provider (
    id             SERIAL PRIMARY KEY,
    user_id        INTEGER,
    provider_name  VARCHAR(255) NOT NULL,
    base_url       VARCHAR(255) NOT NULL,
    api_key        VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE (user_id, provider_name)
);

CREATE TABLE ai_model(
    id SERIAL PRIMARY KEY,
    provider_id INTEGER,
    model_name VARCHAR(255) NOT NULL,
    FOREIGN KEY (provider_id) REFERENCES ai_provider(id) ON DELETE CASCADE
);

CREATE INDEX idx_ai_model_provider_id ON ai_model (provider_id);