use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Pleito, PleitoFiltro},
};

#[get("/<candidato>/<ano>/<turno>")]
async fn get(
    db: Connection<Database>,
    candidato: String,
    ano: i16,
    turno: i16,
) -> Result<Template, ServerError> {
    let pleito = Pleito::obter(&db, &candidato, ano, turno).await?;
    let ctx = context! {pleito};

    Ok(Template::render("routes/pleito", ctx))
}

#[get("/?<filtro>&<pagina>")]
async fn list(
    db: Connection<Database>,
    filtro: PleitoFiltro,
    pagina: Option<u16>,
) -> Result<Template, ServerError> {
    let pagina = pagina.unwrap_or(1);
    let pleitos = Pleito::listar(&db, filtro.clone(), pagina, 50).await?;
    let ctx = context! {pleitos, filtro, pagina};

    Ok(Template::render("routes/pleitos", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
