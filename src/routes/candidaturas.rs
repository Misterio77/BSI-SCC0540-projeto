/// Rota que exibe info de uma candidatura
use rocket::get;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{Candidatura, CandidaturaFiltro},
};

#[get("/<candidato>/<ano>")]
pub async fn get(
    db: Connection<Database>,
    candidato: String,
    ano: i16,
) -> Result<Template, ServerError> {
    let candidatura = Candidatura::obter(&db, &candidato, ano).await?;
    let ctx = context! {candidatura};
    let template = Template::render("candidatura", ctx);
    Ok(template)
}

#[get("/?<filtro>")]
pub async fn list(
    db: Connection<Database>,
    filtro: CandidaturaFiltro,
) -> Result<Template, ServerError> {
    let candidaturas = Candidatura::listar(&db, filtro.clone()).await?;
    let ctx = context! {candidaturas, filtro};
    let template = Template::render("candidaturas", ctx);
    Ok(template)
}
