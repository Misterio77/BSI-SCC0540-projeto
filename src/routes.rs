use crate::{
    error::Result,
    Database,
    schema::candidatura::Candidatura,
    schema::cargo::{Cargo, TipoCargo},
};

use rocket::{get, routes, Route};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};

#[get("/")]
async fn candidatura(db: Connection<Database>) -> Result<Template> {
    let cargo = Cargo {
        tipo: TipoCargo::DeputadoEstadual,
        local: "São Carlos".into(),
        cadeiras: 21,
    };
    let candidatura = Candidatura::obter(&db, &cargo, 50123, 2020).await?;

    let ctx = context! {
        cargo,
        candidatura,
    };
    Ok(Template::render("candidatura", ctx))
}

pub fn routes() -> Vec<Route> {
    routes![candidatura]
}
