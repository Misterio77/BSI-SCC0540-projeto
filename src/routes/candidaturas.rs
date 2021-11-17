use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Candidatura, CandidaturaFiltro},
};

#[get("/<candidato>/<ano>")]
async fn get(
    db: Connection<Database>,
    candidato: String,
    ano: i16,
) -> Result<Template, ServerError> {
    let candidatura = Candidatura::obter(&db, &candidato, ano).await?;
    let ctx = context! {candidatura};

    Ok(Template::render("rotas/candidatura", ctx))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: CandidaturaFiltro,
) -> Result<Template, ServerError> {
    let candidaturas = Candidatura::listar(&db, filtro.clone()).await?;
    let ctx = context! {candidaturas, filtro};

    Ok(Template::render("rotas/candidaturas", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
