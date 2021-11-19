use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::context;

use crate::{
    database::Database,
    error::ServerError,
    pagination::Pages,
    schema::{Apoio, ApoioFiltro},
    Response,
};

#[get("/<apoiador>/<candidato>/<ano>")]
async fn get(
    db: Connection<Database>,
    apoiador: String,
    candidato: String,
    ano: i16,
) -> Result<Response<Apoio>, ServerError> {
    let apoio = Apoio::obter(&db, &apoiador, &candidato, ano).await?;

    Ok(Response::new(
        apoio.clone(),
        "routes/apoio",
        context! {apoio},
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: ApoioFiltro,
    paginas: Pages,
) -> Result<Response<Vec<Apoio>>, ServerError> {
    let apoios = Apoio::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Response::new(
        apoios.clone(),
        "routes/apoios",
        context! {apoios, filtro, paginas},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
