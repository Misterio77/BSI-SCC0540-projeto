use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Processo, ProcessoFiltro},
    pagination::Pages,
};

#[get("/<id>")]
async fn get(db: Connection<Database>, id: i32) -> Result<Template, ServerError> {
    let processo = Processo::obter(&db, id).await?;
    let ctx = context! {processo};

    Ok(Template::render("routes/processo", ctx))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: ProcessoFiltro,
    paginas: Pages,
) -> Result<Template, ServerError> {
    let processos = Processo::listar(&db, filtro.clone(), paginas.current, 50).await?;
    let ctx = context! {processos, filtro, paginas};

    Ok(Template::render("routes/processos", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
