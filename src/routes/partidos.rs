use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Partido, PartidoFiltro},
    pagination::Pages,
};

#[get("/<numero>")]
async fn get(db: Connection<Database>, numero: i16) -> Result<Template, ServerError> {
    let partido = Partido::obter(&db, numero).await?;
    let ctx = context! {partido};

    Ok(Template::render("routes/partido", ctx))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: PartidoFiltro,
    paginas: Pages,
) -> Result<Template, ServerError> {
    let partidos = Partido::listar(&db, filtro.clone(), paginas.current, 50).await?;
    let ctx = context! {partidos, filtro, paginas};

    Ok(Template::render("routes/partidos", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
