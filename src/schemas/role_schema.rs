use crate::schemas::{
  users,
};

table! {
  roles(id) {
    id -> Text,
    code -> Text,
    name -> Text,
    created_at -> Timestamp,
    updated_at -> Timestamp,
  }
}

table! {
  role_user(id) {
    id -> Text,
    role_id -> Text,
    user_id -> Text,
    created_at -> Timestamp,
  }
}

joinable!(role_user -> roles (role_id));
joinable!(role_user -> users (user_id));

allow_tables_to_appear_in_same_query!(
  roles,
  users,
  role_user,
);
