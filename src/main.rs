use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
use rocket_db_pools::Database as DatabaseTrait;
use sass_rocket_fairing::SassFairing;

use projeto_bd::{
    // Alias para retornar nossos error customizados
    error::Result,
    // Rotas do servidor
    routes::routes,
    // Nosso banco de dados
    Database,
};

#[rocket::main]
async fn main() -> Result<()> {
    rocket::build()
        // Middleware pra automaticamente compilar SASS
        .attach(SassFairing)
        // Middleware pra conexões de database
        .attach(Database::init())
        // Middleware pra gerir templates html
        .attach(Template::fairing())
        // Servir assets da pasta assets (style.css)
        .mount("/assets", FileServer::from(relative!("assets")))
        // Servir rotas
        .mount("/", routes())
        .launch()
        .await?;
    Ok(())
}
