/// Rota que exibe info de um processo
use rocket::get;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{database::Database, error::ServerError, schema::Processo};

#[get("/<id>")]
pub async fn get(db: Connection<Database>, id: i32) -> Result<Template, ServerError> {
    let processo = Processo::obter(&db, id).await?;
    let ctx = context! {processo};

    Ok(Template::render("processo", ctx))
}
