table! {
  api_tokens(id) {
    id -> Text,
    user_id -> Text,
    expired_at -> Nullable<Timestamp>,
    created_at -> Timestamp,
  }
}