CREATE TABLE permission_role (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  permission_id VARCHAR(36) NOT NULL,
  role_id VARCHAR(36) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE permission_role ADD FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE;
ALTER TABLE permission_role ADD FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE;
