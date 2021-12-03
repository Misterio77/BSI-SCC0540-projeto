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
    schema::{Doacao, DoacaoFiltro},
    template_or_json::TemplateOrJson,
};

#[get("/<id>")]
async fn get(db: Connection<Database>, id: i32) -> Result<TemplateOrJson<Doacao>, ServerError> {
    let doacao = Doacao::obter(&db, id).await?;

    let json = Json(doacao.clone());
    let template = Template::render("routes/doacao", context! {doacao});
    Ok(TemplateOrJson(template, json))
}

#[delete("/<id>")]
async fn delete(db: Connection<Database>, id: i32) -> Result<Flash<Redirect>, ServerError> {
    let doacao = Doacao::obter(&db, id).await?;
    doacao.remover(&db).await?;

    Ok(Flash::success(
        Redirect::to("/doacoes"),
        "Remoção bem sucedida.",
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    filtro: DoacaoFiltro,
    paginas: Pages,
) -> Result<TemplateOrJson<Vec<Doacao>>, ServerError> {
    let doacoes = Doacao::listar(&db, filtro.clone(), paginas.current, 50).await?;

    let json = Json(doacoes.clone());
    let template = Template::render("routes/doacoes", context! {doacoes, filtro, paginas, flash});
    Ok(TemplateOrJson(template, json))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
