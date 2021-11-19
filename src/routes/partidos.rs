use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::context;

use crate::{
    database::Database,
    error::ServerError,
    pagination::Pages,
    schema::{Partido, PartidoFiltro},
    Response,
};

#[get("/<numero>")]
async fn get(db: Connection<Database>, numero: i16) -> Result<Response<Partido>, ServerError> {
    let partido = Partido::obter(&db, numero).await?;

    Ok(Response::new(
        partido.clone(),
        "routes/partido",
        context! {partido},
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: PartidoFiltro,
    paginas: Pages,
) -> Result<Response<Vec<Partido>>, ServerError> {
    let partidos = Partido::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Response::new(
        partidos.clone(),
        "routes/partidos",
        context! {partidos, filtro, paginas},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
