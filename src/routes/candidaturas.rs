use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

use crate::schema::Candidatura;
use crate::database::Database;
use crate::error::Result;

#[get("/<ano>/<candidato_cpf>")]
async fn candidatura(db: Connection<Database>) -> Result<Template> {
}

pub fn routes() -> Vec<Route> {
    routes![candidaturas]
}
