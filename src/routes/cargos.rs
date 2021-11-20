use rocket::{
    delete, get,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes, Route,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use std::str::FromStr;

use crate::{
    database::Database,
    error::ServerError,
    pagination::Pages,
    schema::{Cargo, CargoFiltro, TipoCargo},
};

#[get("/<tipo>/<local>")]
async fn get(
    db: Connection<Database>,
    tipo: String,
    local: String,
) -> Result<Template, ServerError> {
    let cargo = Cargo::obter(&db, TipoCargo::from_str(&tipo)?, &local).await?;

    Ok(Template::render("routes/cargo", context! {cargo}))
}

#[delete("/<tipo>/<local>")]
async fn delete(
    db: Connection<Database>,
    tipo: String,
    local: String,
) -> Result<Flash<Redirect>, ServerError> {
    let cargo = Cargo::obter(&db, TipoCargo::from_str(&tipo)?, &local).await?;
    cargo.remover(&db).await?;

    Ok(Flash::success(
        Redirect::to("/cargos"),
        "Remoção bem sucedida.",
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    filtro: CargoFiltro,
    paginas: Pages,
) -> Result<Template, ServerError> {
    let cargos = Cargo::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Template::render(
        "routes/cargos",
        context! {cargos, filtro, paginas, flash},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
