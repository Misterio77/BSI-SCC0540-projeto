use rocket_db_pools::{deadpool_postgres, Database as DatabaseTrait};

/// Reexportar Row e Client do postgres
pub use deadpool_postgres::{tokio_postgres::row::Row, ClientWrapper as Client};

/// Nosso tipo representando a conexão com a de dados
#[derive(DatabaseTrait)]
#[database("database")]
pub struct Database(rocket_db_pools::deadpool_postgres::Pool);
