CREATE TABLE roles (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  code VARCHAR NOT NULL UNIQUE,
  name VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_role_code ON roles(code);
CREATE INDEX idx_role_name ON roles(name);
CREATE INDEX idx_role_code_and_name ON roles(code, name);
