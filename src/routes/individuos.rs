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
    schema::{Individuo, IndividuoFiltro},
    template_or_json::TemplateOrJson,
};

#[get("/<cpfcnpj>")]
async fn get(
    db: Connection<Database>,
    cpfcnpj: String,
) -> Result<TemplateOrJson<Individuo>, ServerError> {
    let individuo = Individuo::obter(&db, &cpfcnpj).await?;

    let json = Json(individuo.clone());
    let template = Template::render("routes/individuo", context! {individuo});
    Ok(TemplateOrJson(template, json))
}

#[delete("/<cpfcnpj>")]
async fn delete(db: Connection<Database>, cpfcnpj: String) -> Result<Flash<Redirect>, ServerError> {
    let individuo = Individuo::obter(&db, &cpfcnpj).await?;
    individuo.remover(&db).await?;

    Ok(Flash::success(
        Redirect::to("/individuos"),
        "Remoção bem sucedida.",
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    filtro: IndividuoFiltro,
    paginas: Pages,
) -> Result<TemplateOrJson<Vec<Individuo>>, ServerError> {
    let individuos = Individuo::listar(&db, filtro.clone(), paginas.current, 50).await?;

    let json = Json(individuos.clone());
    let template = Template::render(
        "routes/individuos",
        context! {individuos, filtro, paginas, flash},
    );
    Ok(TemplateOrJson(template, json))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
