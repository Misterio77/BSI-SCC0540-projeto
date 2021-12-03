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
    schema::{Julgamento, JulgamentoFiltro},
    template_or_json::TemplateOrJson,
};

#[get("/<processo>/<instancia>")]
async fn get(
    db: Connection<Database>,
    processo: i32,
    instancia: String,
) -> Result<TemplateOrJson<Julgamento>, ServerError> {
    let julgamento = Julgamento::obter(&db, processo, &instancia).await?;

    let json = Json(julgamento.clone());
    let template = Template::render("routes/julgamento", context! {julgamento});
    Ok(TemplateOrJson(template, json))
}

#[delete("/<processo>/<instancia>")]
async fn delete(
    db: Connection<Database>,
    processo: i32,
    instancia: String,
) -> Result<Flash<Redirect>, ServerError> {
    let julgamento = Julgamento::obter(&db, processo, &instancia).await?;
    julgamento.remover(&db).await?;

    Ok(Flash::success(
        Redirect::to("/julgamentos"),
        "Remoção bem sucedida.",
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    filtro: JulgamentoFiltro,
    paginas: Pages,
) -> Result<TemplateOrJson<Vec<Julgamento>>, ServerError> {
    let julgamentos = Julgamento::listar(&db, filtro.clone(), paginas.current, 50).await?;

    let json = Json(julgamentos.clone());
    let template = Template::render(
        "routes/julgamentos",
        context! {julgamentos, filtro, paginas, flash},
    );
    Ok(TemplateOrJson(template, json))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
