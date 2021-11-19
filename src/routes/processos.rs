use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::context;

use crate::{
    database::Database,
    error::ServerError,
    pagination::Pages,
    schema::{Processo, ProcessoFiltro},
    Response,
};

#[get("/<id>")]
async fn get(db: Connection<Database>, id: i32) -> Result<Response<Processo>, ServerError> {
    let processo = Processo::obter(&db, id).await?;

    Ok(Response::new(
        processo.clone(),
        "routes/processo",
        context! {processo},
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: ProcessoFiltro,
    paginas: Pages,
) -> Result<Response<Vec<Processo>>, ServerError> {
    let processos = Processo::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Response::new(
        processos.clone(),
        "routes/processos",
        context! {processos, filtro, paginas},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
