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
    schema::{Individuo, IndividuoFiltro},
};

#[get("/<cpfcnpj>")]
async fn get(db: Connection<Database>, cpfcnpj: String) -> Result<Template, ServerError> {
    let individuo = Individuo::obter(&db, &cpfcnpj).await?;

    Ok(Template::render("routes/individuo", context! {individuo}))
}

#[delete("/<cpfcnpj>")]
async fn delete(
    db: Connection<Database>,
    cpfcnpj: String,
) -> Result<Flash<Redirect>, ServerError> {
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
) -> Result<Template, ServerError> {
    let individuos = Individuo::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Template::render(
        "routes/individuos",
        context! {individuos, filtro, paginas, flash},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list, delete]
}
