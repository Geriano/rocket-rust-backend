CREATE TABLE permission_user (
  id VARCHAR(36) PRIMARY KEY NOT NULL,
  permission_id VARCHAR(36) NOT NULL,
  user_id VARCHAR(36) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE permission_user ADD FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE;
ALTER TABLE permission_user ADD FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
