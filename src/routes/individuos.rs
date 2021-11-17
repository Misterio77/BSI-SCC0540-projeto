use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Individuo, IndividuoFiltro},
};

#[get("/<cpfcnpj>")]
pub async fn get(db: Connection<Database>, cpfcnpj: String) -> Result<Template, ServerError> {
    let individuo = Individuo::obter(&db, &cpfcnpj).await?;
    let ctx = context! {individuo};

    Ok(Template::render("rotas/individuo", ctx))
}

#[get("/?<filtro>")]
pub async fn list(
    db: Connection<Database>,
    filtro: IndividuoFiltro,
) -> Result<Template, ServerError> {
    let individuos = Individuo::listar(&db, filtro.clone()).await?;
    let ctx = context! {individuos, filtro};

    Ok(Template::render("rotas/individuos", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
