use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Julgamento, JulgamentoFiltro},
};

#[get("/<processo>/<instancia>")]
async fn get(
    db: Connection<Database>,
    processo: i32,
    instancia: String,
) -> Result<Template, ServerError> {
    let julgamento = Julgamento::obter(&db, processo, &instancia).await?;
    let ctx = context! {julgamento};

    Ok(Template::render("routes/julgamento", ctx))
}

#[get("/?<filtro>&<pagina>")]
async fn list(
    db: Connection<Database>,
    filtro: JulgamentoFiltro,
    pagina: Option<u16>,
) -> Result<Template, ServerError> {
    let pagina = pagina.unwrap_or(1);
    let julgamentos = Julgamento::listar(&db, filtro.clone(), pagina, 50).await?;
    let ctx = context! {julgamentos, filtro, pagina};

    Ok(Template::render("routes/julgamentos", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
