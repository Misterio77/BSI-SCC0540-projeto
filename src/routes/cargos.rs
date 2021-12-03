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
    schema::{Cargo, CargoFiltro, TipoCargo},
    template_or_json::TemplateOrJson,
};

#[get("/<tipo>/<local>")]
async fn get(
    db: Connection<Database>,
    tipo: TipoCargo,
    local: String,
) -> Result<TemplateOrJson<Cargo>, ServerError> {
    let cargo = Cargo::obter(&db, tipo, &local).await?;

    let json = Json(cargo.clone());
    let template = Template::render("routes/cargo", context! {cargo});
    Ok(TemplateOrJson(template, json))
}

#[delete("/<tipo>/<local>")]
async fn delete(
    db: Connection<Database>,
    tipo: TipoCargo,
    local: String,
) -> Result<Flash<Redirect>, ServerError> {
    let cargo = Cargo::obter(&db, tipo, &local).await?;
    cargo.remover(&db).await?;

    Ok(Flash::success(
        Redirect::to("/cargos"),
        "Remoção bem sucedida.",
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    filtro: CargoFiltro,
    paginas: Pages,
) -> Result<TemplateOrJson<Vec<Cargo>>, ServerError> {
    let cargos = Cargo::listar(&db, filtro.clone(), paginas.current, 50).await?;

    let json = Json(cargos.clone());
    let template = Template::render("routes/cargos", context! {cargos, filtro, paginas, flash});
    Ok(TemplateOrJson(template, json))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
