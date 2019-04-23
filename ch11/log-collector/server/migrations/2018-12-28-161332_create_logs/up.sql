-- Your SQL goes here
CREATE TABLE logs (
  id BIGSERIAL NOT NULL,
  user_agent VARCHAR NOT NULL,
  response_time INT NOT NULL,
  timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  PRIMARY KEY (id)
);

