use rocket::{get, routes, Route};
use rocket_db_pools::Connection;

use crate::schema::cargo::{Cargo, TipoCargo};
use crate::{database::Database, error::Result};

#[get("/")]
async fn candidaturas(db: Connection<Database>) -> Result<()> {
    let cargos = Cargo::listar(
        &db,
        Some(&TipoCargo::DeputadoFederal),
        Some("São Paulo"),
        None,
    )
    .await?;
    println!("{:?}", cargos);
    Ok(())
}

pub fn routes() -> Vec<Route> {
    routes![candidaturas]
}
