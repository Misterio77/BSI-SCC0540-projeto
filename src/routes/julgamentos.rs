use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::context;

use crate::{
    database::Database,
    error::ServerError,
    pagination::Pages,
    schema::{Julgamento, JulgamentoFiltro},
    Response,
};

#[get("/<processo>/<instancia>")]
async fn get(
    db: Connection<Database>,
    processo: i32,
    instancia: String,
) -> Result<Response<Julgamento>, ServerError> {
    let julgamento = Julgamento::obter(&db, processo, &instancia).await?;

    Ok(Response::new(
        julgamento.clone(),
        "routes/julgamento",
        context! {julgamento},
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: JulgamentoFiltro,
    paginas: Pages,
) -> Result<Response<Vec<Julgamento>>, ServerError> {
    let julgamentos = Julgamento::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Response::new(
        julgamentos.clone(),
        "routes/julgamentos",
        context! {julgamentos, filtro, paginas},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
