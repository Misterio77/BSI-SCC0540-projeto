use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    pagination::Pages,
    schema::{Processo, ProcessoFiltro},
};

#[get("/<id>")]
async fn get(db: Connection<Database>, id: i32) -> Result<Template, ServerError> {
    let processo = Processo::obter(&db, id).await?;

    Ok(Template::render("routes/processo", context! {processo}))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: ProcessoFiltro,
    paginas: Pages,
) -> Result<Template, ServerError> {
    let processos = Processo::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Template::render(
        "routes/processos",
        context! {processos, filtro, paginas},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
