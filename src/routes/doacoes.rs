use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Doacao, DoacaoFiltro},
    pagination::Pages,
};

#[get("/<id>")]
async fn get(db: Connection<Database>, id: i32) -> Result<Template, ServerError> {
    let doacao = Doacao::obter(&db, id).await?;
    let ctx = context! {doacao};

    Ok(Template::render("routes/doacao", ctx))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: DoacaoFiltro,
    paginas: Pages,
) -> Result<Template, ServerError> {
    let doacoes = Doacao::listar(&db, filtro.clone(), paginas.current, 50).await?;
    let ctx = context! {doacoes, filtro, paginas};

    Ok(Template::render("routes/doacoes", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
