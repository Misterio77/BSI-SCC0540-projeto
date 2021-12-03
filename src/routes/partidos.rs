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
    schema::{Partido, PartidoFiltro},
    template_or_json::TemplateOrJson,
};

#[get("/<numero>")]
async fn get(
    db: Connection<Database>,
    numero: i16,
) -> Result<TemplateOrJson<Partido>, ServerError> {
    let partido = Partido::obter(&db, numero).await?;

    let json = Json(partido.clone());
    let template = Template::render("routes/partido", context! {partido});
    Ok(TemplateOrJson(template, json))
}

#[delete("/<numero>")]
async fn delete(db: Connection<Database>, numero: i16) -> Result<Flash<Redirect>, ServerError> {
    let partido = Partido::obter(&db, numero).await?;
    partido.remover(&db).await?;

    Ok(Flash::success(
        Redirect::to("/partidos"),
        "Remoção bem sucedida.",
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    filtro: PartidoFiltro,
    paginas: Pages,
) -> Result<TemplateOrJson<Vec<Partido>>, ServerError> {
    let partidos = Partido::listar(&db, filtro.clone(), paginas.current, 50).await?;

    let json = Json(partidos.clone());
    let template = Template::render(
        "routes/partidos",
        context! {partidos, filtro, paginas, flash},
    );
    Ok(TemplateOrJson(template, json))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
