use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Partido, PartidoFiltro},
};

#[get("/<numero>")]
pub async fn get(db: Connection<Database>, numero: i16) -> Result<Template, ServerError> {
    let partido = Partido::obter(&db, numero).await?;
    let ctx = context! {partido};

    Ok(Template::render("rotas/partido", ctx))
}

#[get("/?<filtro>")]
pub async fn list(
    db: Connection<Database>,
    filtro: PartidoFiltro,
) -> Result<Template, ServerError> {
    let partidos = Partido::listar(&db, filtro.clone()).await?;
    let ctx = context! {partidos, filtro};

    Ok(Template::render("rotas/partidos", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
