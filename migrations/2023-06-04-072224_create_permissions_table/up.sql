CREATE TABLE permissions (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  code VARCHAR NOT NULL UNIQUE,
  name VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_permission_code ON permissions(code);
CREATE INDEX idx_permission_name ON permissions(name);
CREATE INDEX idx_permission_code_and_name ON permissions(code, name);
