/// Rota que exibe info de um indivíduo
use rocket::get;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{database::Database, error::ServerError, schema::Individuo};

#[get("/<cpfcnpj>")]
pub async fn get(db: Connection<Database>, cpfcnpj: String) -> Result<Template, ServerError> {
    let individuo = Individuo::obter(&db, &cpfcnpj).await?;
    let ctx = context! {individuo};

    Ok(Template::render("individuo", ctx))
}
