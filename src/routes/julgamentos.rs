use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Julgamento, JulgamentoFiltro},
};

#[get("/<processo>/<instancia>")]
pub async fn get(
    db: Connection<Database>,
    processo: i32,
    instancia: String,
) -> Result<Template, ServerError> {
    let julgamento = Julgamento::obter(&db, processo, &instancia).await?;
    let ctx = context! {julgamento};

    Ok(Template::render("routes/julgamento", ctx))
}

#[get("/?<filtro>")]
pub async fn list(
    db: Connection<Database>,
    filtro: JulgamentoFiltro,
) -> Result<Template, ServerError> {
    let julgamentos = Julgamento::listar(&db, filtro.clone()).await?;
    let ctx = context! {julgamentos, filtro};

    Ok(Template::render("routes/julgamentos", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
