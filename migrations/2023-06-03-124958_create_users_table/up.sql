CREATE TABLE users (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL UNIQUE,
  username VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL,
  email_verified_at TIMESTAMP NULL DEFAULT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMP NULL DEFAULT NULL
);

CREATE INDEX users_01_idx ON users (name);
CREATE INDEX users_02_idx ON users (email);
CREATE INDEX users_03_idx ON users (username);
CREATE INDEX users_04_idx ON users (email_verified_at);
CREATE INDEX users_05_idx ON users (created_at);
CREATE INDEX users_06_idx ON users (updated_at);
CREATE INDEX users_07_idx ON users (deleted_at);

CREATE INDEX users_08_idx ON users (name, email);
CREATE INDEX users_09_idx ON users (name, username);
CREATE INDEX users_10_idx ON users (name, email_verified_at);
CREATE INDEX users_11_idx ON users (name, created_at);
CREATE INDEX users_12_idx ON users (name, updated_at);
CREATE INDEX users_13_idx ON users (name, deleted_at);

CREATE INDEX users_14_idx ON users (email, username);
CREATE INDEX users_15_idx ON users (email, email_verified_at);
CREATE INDEX users_16_idx ON users (email, created_at);
CREATE INDEX users_17_idx ON users (email, updated_at);
CREATE INDEX users_18_idx ON users (email, deleted_at);

CREATE INDEX users_19_idx ON users (username, emaiL_verified_at);
CREATE INDEX users_20_idx ON users (username, created_at);
CREATE INDEX users_21_idx ON users (username, updated_at);
CREATE INDEX users_22_idx ON users (username, deleted_at);

CREATE INDEX users_23_idx ON users (email_verified_at, created_at);
CREATE INDEX users_24_idx ON users (email_verified_at, updated_at);
CREATE INDEX users_25_idx ON users (email_verified_at, deleted_at);

CREATE INDEX users_26_idx ON users (name, email, username);
CREATE INDEX users_27_idx ON users (name, email, email_verified_at);
CREATE INDEX users_28_idx ON users (name, email, created_at);
CREATE INDEX users_29_idx ON users (name, email, updated_at);
CREATE INDEX users_30_idx ON users (name, email, deleted_at);

CREATE INDEX users_31_idx ON users (name, email, username, email_verified_at);
CREATE INDEX users_32_idx ON users (name, email, username, created_at);
CREATE INDEX users_33_idx ON users (name, email, username, updated_at);
CREATE INDEX users_34_idx ON users (name, email, username, deleted_at);

CREATE INDEX users_35_idx ON users (name, email, username, email_verified_at, created_at);
CREATE INDEX users_36_idx ON users (name, email, username, email_verified_at, updated_at);
CREATE INDEX users_37_idx ON users (name, email, username, email_verified_at, deleted_at);

CREATE INDEX users_38_idx ON users (name, email, username, email_verified_at, created_at, updated_at);
CREATE INDEX users_39_idx ON users (name, email, username, email_verified_at, created_at, deleted_at);

CREATE INDEX users_40_idx ON users (name, email, username, email_verified_at, created_at, updated_at, deleted_at);
