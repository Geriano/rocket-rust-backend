use crate::schemas::{
  roles,
  users,
};

table! {
  permissions(id) {
    id -> Text,
    code -> Text,
    name -> Text,
    created_at -> Timestamp,
    updated_at -> Timestamp,
  }
}

table! {
  permission_user(id) {
    id -> Text,
    permission_id -> Text,
    user_id -> Text,
    created_at -> Timestamp,
  }
}

table! {
  permission_role(id) {
    id -> Text,
    permission_id -> Text,
    role_id -> Text,
    created_at -> Timestamp,
  }
}

joinable!(permission_role -> permissions (permission_id));
joinable!(permission_role -> roles (role_id));

allow_tables_to_appear_in_same_query!(
  permissions,
  roles,
  permission_role,
);

joinable!(permission_user -> permissions (permission_id));
joinable!(permission_user -> users (user_id));

allow_tables_to_appear_in_same_query!(
  permissions,
  users,
  permission_user,
);
