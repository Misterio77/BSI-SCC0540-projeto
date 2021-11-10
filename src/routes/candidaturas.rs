use rocket::{get, http::uri::Origin, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::database::Database;
use crate::error::ServerError;
use crate::schema::Candidatura;

#[get("/<ano>/<cpfcnpj>")]
async fn candidatura(
    db: Connection<Database>,
    origin: &Origin<'_>,
    ano: i16,
    cpfcnpj: String,
) -> Result<Template, ServerError> {
    let candidatura = Candidatura::obter(&db, &cpfcnpj, ano).await?;
    // .map_err(|e| ServerError::from(e).flash_redirect("/candidaturas"))?;

    Ok(Template::render(
        "candidatura",
        context! {
            origin,
            candidatura,
        },
    ))
}

pub fn routes() -> Vec<Route> {
    routes![candidatura]
}
