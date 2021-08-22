-- Your SQL goes here
CREATE TABLE Customer (
    id              BIGSERIAL   PRIMARY KEY,
    mail            VARCHAR     NOT NULL UNIQUE,
    pass_hash       VARCHAR     NOT NULL,
    register_data   TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

