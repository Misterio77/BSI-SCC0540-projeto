/// Rota que exibe info de um julgamento
use rocket::get;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{database::Database, error::ServerError, schema::Julgamento};

#[get("/<processo>/<instancia>")]
pub async fn get(
    db: Connection<Database>,
    processo: i32,
    instancia: String,
) -> Result<Template, ServerError> {
    let julgamento = Julgamento::obter(&db, processo, &instancia).await?;
    let ctx = context! {julgamento};

    Ok(Template::render("julgamento", ctx))
}
