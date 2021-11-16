/// Rota que exibe info de um partido
use rocket::get;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{database::Database, error::ServerError, schema::Partido};

#[get("/<numero>")]
pub async fn get(db: Connection<Database>, numero: i16) -> Result<Template, ServerError> {
    let partido = Partido::obter(&db, numero).await?;
    let ctx = context! {partido};

    Ok(Template::render("partido", ctx))
}
