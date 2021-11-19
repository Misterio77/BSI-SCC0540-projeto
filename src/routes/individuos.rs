use rocket::{
    delete, get,
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
async fn delete(db: Connection<Database>, cpfcnpj: String) -> Result<Flash<Redirect>, ServerError> {
    let individuo = Individuo::obter(&db, &cpfcnpj);
    individuo.remover().await?;

    Ok(Flash::success(
        Redirect::to("/individuos"),
        "Remoção realizada com sucesso",
    ))
}

#[get("/?<filtro>")]
async fn list(
    db: Connection<Database>,
    filtro: IndividuoFiltro,
    paginas: Pages,
) -> Result<Template, ServerError> {
    let individuos = Individuo::listar(&db, filtro.clone(), paginas.current, 50).await?;

    Ok(Template::render(
        "routes/individuos",
        context! {individuos, filtro, paginas},
    ))
}

pub fn routes() -> Vec<Route> {
    routes![get, list]
}
