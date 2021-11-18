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

    Ok(Template::render("routes/candidatura", ctx))
}

#[get("/?<filtro>&<pagina>")]
async fn list(
    db: Connection<Database>,
    filtro: CandidaturaFiltro,
    pagina: Option<u16>,
) -> Result<Template, ServerError> {
    let pagina = pagina.unwrap_or(1);
    let candidaturas = Candidatura::listar(&db, filtro.clone(), pagina, 50).await?;
    let ctx = context! {candidaturas, filtro, pagina};

    Ok(Template::render("routes/candidaturas", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
