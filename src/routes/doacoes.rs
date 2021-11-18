use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Doacao, DoacaoFiltro},
};

#[get("/<id>")]
async fn get(db: Connection<Database>, id: i32) -> Result<Template, ServerError> {
    let doacao = Doacao::obter(&db, id).await?;
    let ctx = context! {doacao};

    Ok(Template::render("routes/doacao", ctx))
}

#[get("/?<filtro>&<pagina>")]
async fn list(
    db: Connection<Database>,
    filtro: DoacaoFiltro,
    pagina: Option<u16>,
) -> Result<Template, ServerError> {
    let pagina = pagina.unwrap_or(1);
    let doacoes = Doacao::listar(&db, filtro.clone(), pagina, 50).await?;
    let ctx = context! {doacoes, filtro, pagina};

    Ok(Template::render("routes/doacoes", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
