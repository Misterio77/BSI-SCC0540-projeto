pub mod error;
pub mod routes;
pub mod schema;

use rocket_db_pools::{deadpool_postgres, Database as DatabaseTrait};

#[derive(DatabaseTrait)]
#[database("database")]
pub struct Database(deadpool_postgres::Pool);
pub use deadpool_postgres::{
    ClientWrapper as DatabaseClient,
    tokio_postgres::row::Row as DatabaseRow,
};
