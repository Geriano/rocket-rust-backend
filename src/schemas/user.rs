table! {
  users (id) {
    id -> Text,
    name -> Text,
    email -> Text,
    email_verified_at -> Nullable<Timestamp>,
    username -> Text,
    password -> Text,
    created_at -> Timestamp,
    updated_at -> Timestamp,
    deleted_at -> Nullable<Timestamp>,
  }
}