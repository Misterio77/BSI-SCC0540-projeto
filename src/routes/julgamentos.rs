use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Julgamento, JulgamentoFiltro},
    pagination::Pages,
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

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: JulgamentoFiltro,
    paginas: Pages,
) -> Result<Template, ServerError> {
    let julgamentos = Julgamento::listar(&db, filtro.clone(), paginas.current, 50).await?;
    let ctx = context! {julgamentos, filtro, paginas};

    Ok(Template::render("routes/julgamentos", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
