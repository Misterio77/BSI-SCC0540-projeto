use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::schema::cargo::{Cargo, TipoCargo};
use crate::{database::Database, error::Result};

#[get("/")]
async fn candidaturas(db: Connection<Database>) -> Result<Template> {
    let cargos = Cargo::listar(
        &db,
        Some(&TipoCargo::DeputadoFederal),
        Some("São Paulo"),
        None,
    )
    .await?;
    Ok(Template::render("base", context! {
        cargos,
    }))
}

pub fn routes() -> Vec<Route> {
    routes![candidaturas]
}
