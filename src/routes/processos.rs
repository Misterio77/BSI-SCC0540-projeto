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
    schema::{Processo, ProcessoFiltro},
};

#[get("/<id>")]
async fn get(db: Connection<Database>, id: i32) -> Result<Template, ServerError> {
    let processo = Processo::obter(&db, id).await?;

    Ok(Template::render("routes/processo", context! {processo}))
}

#[delete("/<id>")]
async fn delete(
    db: Connection<Database>,
    id: i32,
) -> Result<Flash<Redirect>, ServerError> {
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
) -> Result<Template, ServerError> {
    let processos = Processo::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Template::render(
        "routes/processos",
        context! {processos, filtro, paginas, flash},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
