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
    schema::{Pleito, PleitoFiltro},
    template_or_json::TemplateOrJson,
};

#[get("/<candidato>/<ano>/<turno>")]
async fn get(
    db: Connection<Database>,
    candidato: String,
    ano: i16,
    turno: i16,
) -> Result<TemplateOrJson<Pleito>, ServerError> {
    let pleito = Pleito::obter(&db, &candidato, ano, turno).await?;

    let json = Json(pleito.clone());
    let template = Template::render("routes/pleito", context! {pleito});
    Ok(TemplateOrJson(template, json))
}

#[delete("/<candidato>/<ano>/<turno>")]
async fn delete(
    db: Connection<Database>,
    candidato: String,
    ano: i16,
    turno: i16,
) -> Result<Flash<Redirect>, ServerError> {
    let pleito = Pleito::obter(&db, &candidato, ano, turno).await?;
    pleito.remover(&db).await?;

    Ok(Flash::success(
        Redirect::to("/pleitos"),
        "Remoção bem sucedida.",
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    filtro: PleitoFiltro,
    paginas: Pages,
) -> Result<TemplateOrJson<Vec<Pleito>>, ServerError> {
    let pleitos = Pleito::listar(&db, filtro.clone(), paginas.current, 50).await?;

    let json = Json(pleitos.clone());
    let template = Template::render("routes/pleitos", context! {pleitos, filtro, paginas, flash});
    Ok(TemplateOrJson(template, json))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
