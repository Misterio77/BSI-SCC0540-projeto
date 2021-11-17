use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{database::Database, error::ServerError, schema::{Pleito, PleitoFiltro}};

#[get("/<candidato>/<ano>/<turno>")]
pub async fn get(
    db: Connection<Database>,
    candidato: String,
    ano: i16,
    turno: i16,
) -> Result<Template, ServerError> {
    let pleito = Pleito::obter(&db, &candidato, ano, turno).await?;
    let ctx = context! {pleito};

    Ok(Template::render("rotas/pleito", ctx))
}

#[get("/?<filtro>")]
pub async fn list(db: Connection<Database>, filtro: PleitoFiltro) -> Result<Template, ServerError> {
    let pleitos = Pleito::listar(&db, filtro.clone()).await?;
    let ctx = context! {pleitos, filtro};

    Ok(Template::render("rotas/pleitos", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
