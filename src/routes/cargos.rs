/// Rota que exibe info de uma cargo
use std::str::FromStr;
use rocket::get;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::{
    database::Database,
    error::ServerError,
    schema::{TipoCargo, Cargo},
};

#[get("/<tipo>/<local>")]
pub async fn get(db: Connection<Database>, tipo: String, local: String) -> Result<Template, ServerError> {
    let cargo = Cargo::obter(&db, TipoCargo::from_str(&tipo)?, &local).await?;
    let ctx = context!{cargo};

    Ok(Template::render("cargo", ctx))
}