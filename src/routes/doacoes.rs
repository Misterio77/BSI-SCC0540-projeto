use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Doacao, DoacaoFiltro},
};

#[get("/<id>")]
pub async fn get(db: Connection<Database>, id: i32) -> Result<Template, ServerError> {
    let doacao = Doacao::obter(&db, id).await?;
    let ctx = context! {doacao};

    Ok(Template::render("rotas/doacao", ctx))
}

#[get("/?<filtro>")]
pub async fn list(db: Connection<Database>, filtro: DoacaoFiltro) -> Result<Template, ServerError> {
    let doacoes = Doacao::listar(&db, filtro.clone()).await?;
    let ctx = context! {doacoes, filtro};

    Ok(Template::render("rotas/doacoes", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
