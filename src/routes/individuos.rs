use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::context;

use crate::{
    database::Database,
    error::ServerError,
    pagination::Pages,
    schema::{Individuo, IndividuoFiltro},
    Response,
};

#[get("/<cpfcnpj>")]
async fn get(
    db: Connection<Database>,
    cpfcnpj: String,
) -> Result<Response<Individuo>, ServerError> {
    let individuo = Individuo::obter(&db, &cpfcnpj).await?;

    Ok(Response::new(
        individuo.clone(),
        "routes/individuo",
        context! {individuo},
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: IndividuoFiltro,
    paginas: Pages,
) -> Result<Response<Vec<Individuo>>, ServerError> {
    let individuos = Individuo::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Response::new(
        individuos.clone(),
        "routes/individuos",
        context! {individuos, filtro, paginas},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
