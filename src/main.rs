use rocket::{get, routes};
use rocket::fs::{FileServer, relative};
use rocket_db_pools::{deadpool_postgres, Database};
use rocket_dyn_templates::{context, Template};
use sass_rocket_fairing::SassFairing;

use projeto_bd::error::Result;

#[derive(Database)]
#[database("database")]
struct Data(deadpool_postgres::Pool);

#[rocket::main]
async fn main() -> Result<()> {
    rocket::build()
        .attach(Data::init())
        .attach(Template::fairing())
        .attach(SassFairing)
        .mount("/assets", FileServer::from(relative!("assets")))
        .mount("/", routes![render_teste])
        .launch()
        .await?;
    Ok(())
}

#[get("/")]
fn render_teste() -> Template {
    let ctx = context! {
    };
    Template::render("base", ctx)
}
