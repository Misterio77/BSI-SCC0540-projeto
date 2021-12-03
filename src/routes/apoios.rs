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
    schema::{Apoio, ApoioFiltro},
    template_or_json::TemplateOrJson,
};

#[get("/<apoiador>/<ano>")]
async fn get(
    db: Connection<Database>,
    apoiador: String,
    ano: i16,
) -> Result<TemplateOrJson<Apoio>, ServerError> {
    let apoio = Apoio::obter(&db, &apoiador, ano).await?;

    let json = Json(apoio.clone());
    let template = Template::render("routes/apoio", context! {apoio});
    Ok(TemplateOrJson(template, json))
}

#[delete("/<apoiador>/<ano>")]
async fn delete(
    db: Connection<Database>,
    apoiador: String,
    ano: i16,
) -> Result<Flash<Redirect>, ServerError> {
    let apoio = Apoio::obter(&db, &apoiador, ano).await?;
    apoio.remover(&db).await?;

    Ok(Flash::success(
        Redirect::to("/apoios"),
        "Remoção bem sucedida.",
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    filtro: ApoioFiltro,
    paginas: Pages,
) -> Result<TemplateOrJson<Vec<Apoio>>, ServerError> {
    let apoios = Apoio::listar(&db, filtro.clone(), paginas.current, 50).await?;

    let json = Json(apoios.clone());
    let template = Template::render("routes/apoios", context! {apoios,filtro,paginas,flash});
    Ok(TemplateOrJson(template, json))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
