use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    pagination::Pages,
    schema::{Doacao, DoacaoFiltro},
};

#[get("/<id>")]
async fn get(db: Connection<Database>, id: i32) -> Result<Template, ServerError> {
    let doacao = Doacao::obter(&db, id).await?;

    Ok(Template::render("routes/doacao", context! {doacao}))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: DoacaoFiltro,
    paginas: Pages,
) -> Result<Template, ServerError> {
    let doacoes = Doacao::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Template::render(
        "routes/doacoes",
        context! {doacoes, filtro, paginas},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
