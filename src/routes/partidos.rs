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
    schema::{Partido, PartidoFiltro},
};

#[get("/<numero>")]
async fn get(db: Connection<Database>, numero: i16) -> Result<Template, ServerError> {
    let partido = Partido::obter(&db, numero).await?;

    Ok(Template::render("routes/partido", context! {partido}))
}

#[delete("/<numero>")]
async fn delete(
    db: Connection<Database>,
    numero: i16,
) -> Result<Flash<Redirect>, ServerError> {
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
) -> Result<Template, ServerError> {
    let partidos = Partido::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Template::render(
        "routes/partidos",
        context! {partidos, filtro, paginas, flash},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
