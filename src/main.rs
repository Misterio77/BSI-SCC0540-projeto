use rocket::fs::{relative, FileServer};
use rocket_db_pools::Database as DatabaseTrait;
use rocket_dyn_templates::Template;
use sass_rocket_fairing::SassFairing;

use projeto_bd::{
    // Alias para retornar nossos error customizados
    error::Result,
    // Rotas do servidor
    routes::routes,
    // Nosso banco de dados
    database::Database,
};

#[rocket::main]
async fn main() -> Result<()> {
    rocket::build()
        // Middleware pra conexões de database
        .attach(Database::init())
        // Middleware pra gerir templates html
        .attach(Template::fairing())
        // Middleware pra automaticamente compilar SASS
        .attach(SassFairing)
        // Servir assets da pasta assets (style.css)
        .mount("/assets", FileServer::from(relative!("assets")))
        // Servir rotas
        .mount("/", routes())
        .launch()
        .await?;
    Ok(())
}
