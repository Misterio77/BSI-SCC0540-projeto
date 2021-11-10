use rocket::{catchers, routes};
use rocket_db_pools::Database as DatabaseTrait;
use rocket_dyn_templates::Template;

use projeto_bd::{
    // Nosso banco de dados
    database::Database,
    // Nosso tipo personalizado de erro
    error::ServerError,
    // Rotas do servidor
    routes::{candidaturas, css, index, not_found},
};

#[rocket::main]
async fn main() -> Result<(), ServerError> {
    rocket::build()
        // Middleware pra conexões de database
        .attach(Database::init())
        // Middleware pra gerir templates html
        .attach(Template::fairing())
        // Servir assets da pasta assets (style.css)
        .mount("/assets", routes![css])
        // Páginas de erro
        .register("/", catchers![not_found])
        // Servir rotas
        .mount("/", routes![index])
        .mount("/candidaturas", candidaturas::routes())
        .launch()
        .await?;
    Ok(())
}
