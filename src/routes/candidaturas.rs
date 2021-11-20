use rocket::{
    delete, get,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes, Route,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    pagination::Pages,
    schema::{Candidatura, CandidaturaFiltro},
};

#[get("/<candidato>/<ano>")]
async fn get(
    db: Connection<Database>,
    candidato: String,
    ano: i16,
) -> Result<Template, ServerError> {
    let candidatura = Candidatura::obter(&db, &candidato, ano).await?;

    Ok(Template::render(
        "routes/candidatura",
        context! {candidatura},
    ))
}

#[delete("/<candidato>/<ano>")]
async fn delete(
    db: Connection<Database>,
    candidato: String,
    ano: i16,
) -> Result<Flash<Redirect>, ServerError> {
    let candidatura = Candidatura::obter(&db, &candidato, ano).await?;
    candidatura.remover(&db).await?;

    Ok(Flash::success(
        Redirect::to("/candidaturas"),
        "Remoção bem sucedida.",
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    filtro: CandidaturaFiltro,
    paginas: Pages,
) -> Result<Template, ServerError> {
    let candidaturas = Candidatura::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Template::render(
        "routes/candidaturas",
        context! {candidaturas, filtro, paginas, flash},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
