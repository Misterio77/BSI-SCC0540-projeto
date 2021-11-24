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
    schema::{Apoio, ApoioFiltro},
};

#[get("/<apoiador>/<ano>")]
async fn get(
    db: Connection<Database>,
    apoiador: String,
    ano: i16,
) -> Result<Template, ServerError> {
    let apoio = Apoio::obter(&db, &apoiador, ano).await?;

    Ok(Template::render("routes/apoio", context! {apoio}))
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
) -> Result<Template, ServerError> {
    let apoios = Apoio::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Template::render(
        "routes/apoios",
        context! {apoios, filtro, paginas, flash},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
