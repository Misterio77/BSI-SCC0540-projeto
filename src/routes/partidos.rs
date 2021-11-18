use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Partido, PartidoFiltro},
};

#[get("/<numero>")]
async fn get(db: Connection<Database>, numero: i16) -> Result<Template, ServerError> {
    let partido = Partido::obter(&db, numero).await?;
    let ctx = context! {partido};

    Ok(Template::render("routes/partido", ctx))
}

#[get("/?<filtro>&<pagina>")]
async fn list(
    db: Connection<Database>,
    filtro: PartidoFiltro,
    pagina: Option<u16>,
) -> Result<Template, ServerError> {
    let pagina = pagina.unwrap_or(1);
    let partidos = Partido::listar(&db, filtro.clone(), pagina, 50).await?;
    let ctx = context! {partidos, filtro, pagina};

    Ok(Template::render("routes/partidos", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
