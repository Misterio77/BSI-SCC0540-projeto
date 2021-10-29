use rocket::{get, routes, Route};
use rocket_db_pools::Connection;

use crate::{
    error::Result,
    schema::Individuo,
    database::Database
};

#[get("/")]
async fn candidaturas(db: Connection<Database>) -> Result<()> {
    let individuo = Individuo::obter(&db, "23858708860").await?;
    println!("{:?}", individuo);

    let candidaturas = individuo.candidaturas(&db).await?;
    println!("{:?}", candidaturas);
    Ok(())
}

pub fn routes() -> Vec<Route> {
    routes![candidaturas]
}
