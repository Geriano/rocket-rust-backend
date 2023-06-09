use std::env;
use diesel::{
  r2d2::{
    Pool,
    ConnectionManager,
    PooledConnection
  },
  PgConnection
};

pub type PgManager = ConnectionManager::<PgConnection>;
pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let manager = PgManager::new(database_url);

  Pool::builder().build(manager).expect("Failed to create pool.")
}

pub fn get_conn() -> PgPooledConnection {
  establish_connection().get().expect("Failed to get DB connection.")
}
