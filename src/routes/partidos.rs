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

#[get("/?<filtro>")]
async fn list(db: Connection<Database>, filtro: PartidoFiltro) -> Result<Template, ServerError> {
    let partidos = Partido::listar(&db, filtro.clone()).await?;
    let ctx = context! {partidos, filtro};

    Ok(Template::render("routes/partidos", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
