use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use std::str::FromStr;

use crate::{
    database::Database,
    error::ServerError,
    schema::{Cargo, CargoFiltro, TipoCargo},
};

#[get("/<tipo>/<local>")]
pub async fn get(
    db: Connection<Database>,
    tipo: String,
    local: String,
) -> Result<Template, ServerError> {
    let cargo = Cargo::obter(&db, TipoCargo::from_str(&tipo)?, &local).await?;
    let ctx = context! {cargo};

    Ok(Template::render("rotas/cargo", ctx))
}

#[get("/?<filtro>")]
pub async fn list(db: Connection<Database>, filtro: CargoFiltro) -> Result<Template, ServerError> {
    let cargos = Cargo::listar(&db, filtro.clone()).await?;
    let ctx = context! {cargos, filtro};

    Ok(Template::render("rotas/cargos", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
