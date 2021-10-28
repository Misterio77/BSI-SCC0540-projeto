pub mod error;
pub mod routes;
pub mod schema;

use rocket_db_pools::{deadpool_postgres, Database as DatabaseTrait};

#[derive(DatabaseTrait)]
#[database("database")]
struct Database(deadpool_postgres::Pool);
