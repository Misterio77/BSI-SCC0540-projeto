/// Rota que exibe info de um pleito
use rocket::get;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::Pleito,
};

#[get("/<candidato>/<ano>/<turno>")]
pub async fn get(db: Connection<Database>, candidato: String, ano: i16, turno: i16) -> Result<Template, ServerError> {
    let pleito = Pleito::obter(&db, &candidato, ano, turno).await?;
    let ctx = context!{pleito};

    Ok(Template::render("pleito", ctx))
}
