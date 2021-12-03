use rocket::{
    delete, get,
    request::FlashMessage,
    response::{Flash, Redirect},
    routes,
    serde::json::Json,
    Route,
};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    pagination::Pages,
    schema::{Candidatura, CandidaturaFiltro},
    template_or_json::TemplateOrJson,
};

#[get("/<candidato>/<ano>")]
async fn get(
    db: Connection<Database>,
    candidato: String,
    ano: i16,
) -> Result<TemplateOrJson<Candidatura>, ServerError> {
    let candidatura = Candidatura::obter(&db, &candidato, ano).await?;

    let json = Json(candidatura.clone());
    let template = Template::render("routes/candidatura", context! {candidatura});
    Ok(TemplateOrJson(template, json))
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
) -> Result<TemplateOrJson<Vec<Candidatura>>, ServerError> {
    let candidaturas = Candidatura::listar(&db, filtro.clone(), paginas.current, 50).await?;

    let json = Json(candidaturas.clone());
    let template = Template::render(
        "routes/candidaturas",
        context! {candidaturas, filtro, paginas, flash},
    );
    Ok(TemplateOrJson(template, json))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
