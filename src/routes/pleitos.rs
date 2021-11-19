use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::context;

use crate::{
    database::Database,
    error::ServerError,
    pagination::Pages,
    schema::{Pleito, PleitoFiltro},
    Response,
};

#[get("/<candidato>/<ano>/<turno>")]
async fn get(
    db: Connection<Database>,
    candidato: String,
    ano: i16,
    turno: i16,
) -> Result<Response<Pleito>, ServerError> {
    let pleito = Pleito::obter(&db, &candidato, ano, turno).await?;

    Ok(Response::new(
        pleito.clone(),
        "routes/pleito",
        context! {pleito},
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: PleitoFiltro,
    paginas: Pages,
) -> Result<Response<Vec<Pleito>>, ServerError> {
    let pleitos = Pleito::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Response::new(
        pleitos.clone(),
        "routes/pleitos",
        context! {pleitos, filtro, paginas},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
