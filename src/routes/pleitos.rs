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
    schema::{Pleito, PleitoFiltro},
};

#[get("/<candidato>/<ano>/<turno>")]
async fn get(
    db: Connection<Database>,
    candidato: String,
    ano: i16,
    turno: i16,
) -> Result<Template, ServerError> {
    let pleito = Pleito::obter(&db, &candidato, ano, turno).await?;

    Ok(Template::render("routes/pleito", context! {pleito}))
}

#[delete("/<candidato>/<ano>/<turno>")]
async fn delete(
    db: Connection<Database>,
    candidato: String,
    ano: i16,
    turno: i16,
) -> Result<Flash<Redirect>, ServerError> {
    let pleito = Pleito::obter(&db, &candidato, ano, turno).await?;
    pleito.remover(&db).await?;

    Ok(Flash::success(
        Redirect::to("/pleitos"),
        "Remoção bem sucedida.",
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    flash: Option<FlashMessage<'_>>,
    filtro: PleitoFiltro,
    paginas: Pages,
) -> Result<Template, ServerError> {
    let pleitos = Pleito::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Template::render(
        "routes/pleitos",
        context! {pleitos, filtro, paginas, flash},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
