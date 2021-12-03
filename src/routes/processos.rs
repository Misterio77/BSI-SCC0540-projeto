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
    schema::{Processo, ProcessoFiltro},
    template_or_json::TemplateOrJson,
};

#[get("/<id>")]
async fn get(db: Connection<Database>, id: i32) -> Result<TemplateOrJson<Processo>, ServerError> {
    let processo = Processo::obter(&db, id).await?;

    let json = Json(processo.clone());
    let template = Template::render("routes/processo", context! {processo});
    Ok(TemplateOrJson(template, json))
}

#[delete("/<id>")]
async fn delete(db: Connection<Database>, id: i32) -> Result<Flash<Redirect>, ServerError> {
    let processo = Processo::obter(&db, id).await?;
    processo.remover(&db).await?;

    Ok(Flash::success(
        Redirect::to("/processos"),
        "Remoção bem sucedida.",
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    filtro: ProcessoFiltro,
    paginas: Pages,
) -> Result<TemplateOrJson<Vec<Processo>>, ServerError> {
    let processos = Processo::listar(&db, filtro.clone(), paginas.current, 50).await?;

    let json = Json(processos.clone());
    let template = Template::render(
        "routes/processos",
        context! {processos, filtro, paginas, flash},
    );
    Ok(TemplateOrJson(template, json))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
