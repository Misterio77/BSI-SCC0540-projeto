/// Rota que exibe info de uma doação
use rocket::get;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::Doacao,
};

#[get("/<id>")]
pub async fn get(db: Connection<Database>, id: i32) -> Result<Template, ServerError> {
    let doacao = Doacao::obter(&db, id).await?;
    let ctx = context!{doacao};

    Ok(Template::render("doacao", ctx))
}
