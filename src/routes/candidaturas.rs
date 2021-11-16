use rocket::get;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
/// Rota que exibe info de uma candidatura
use std::str::FromStr;

use crate::{
    database::Database,
    error::ServerError,
    schema::{Candidatura, CandidaturaFiltro, TipoCargo},
};

#[get("/<candidato>/<ano>")]
pub async fn get(
    db: Connection<Database>,
    candidato: String,
    ano: i16,
) -> Result<Template, ServerError> {
    let candidatura = Candidatura::obter(&db, &candidato, ano).await?;
    let ctx = context! {candidatura};

    Ok(Template::render("candidatura", ctx))
}

#[get("/?<candidato>&<vice_candidato>&<ano>&<cargo_tipo>&<cargo_local>&<numero>&<partido>")]
pub async fn list(
    db: Connection<Database>,
    candidato: Option<String>,
    vice_candidato: Option<String>,
    ano: Option<i16>,
    cargo_tipo: Option<String>,
    cargo_local: Option<String>,
    numero: Option<i32>,
    partido: Option<i16>,
) -> Result<Template, ServerError> {
    let cargo_tipo = cargo_tipo
        .as_ref()
        .and_then(|i| TipoCargo::from_str(i).ok());

    let filtro = CandidaturaFiltro {
        candidato: candidato.filter(|s| !s.is_empty()),
        vice_candidato: vice_candidato.filter(|s| !s.is_empty()),
        ano,
        cargo_tipo,
        cargo_local: cargo_local.filter(|s| !s.is_empty()),
        numero,
        partido,
    };

    let candidaturas = Candidatura::listar(&db, &filtro).await?;

    let ctx = context! {candidaturas, filtro};

    Ok(Template::render("candidaturas", ctx))
}
