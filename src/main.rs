use rocket::{catchers, routes};
use rocket_db_pools::Database as DatabaseTrait;
use rocket_dyn_templates::Template;

use projeto_bd::{
    // Nosso banco de dados
    database::Database,
    // Nosso tipo personalizado de erro
    error::ServerError,
    // Rotas do servidor
    routes::{
        assets, candidaturas, cargos, doacoes, errors, home, individuos, julgamentos, partidos,
        pleitos, processos,
    },
};

#[rocket::main]
async fn main() -> Result<(), ServerError> {
    rocket::build()
        // Middleware pra conexões de database
        .attach(Database::init())
        // Middleware pra gerir templates html
        .attach(Template::fairing())
        // Servir assets da pasta assets (style.css)
        .mount("/assets", routes![assets::css])
        // Páginas de erro
        .register("/", catchers![errors::not_found])
        // Servir rotas
        .mount("/", routes![home::index])
        .mount("/candidaturas", candidaturas::routes())
        .mount("/cargos", cargos::routes())
        .mount("/doacoes", doacoes::routes())
        .mount("/individuos", individuos::routes())
        .mount("/partidos", partidos::routes())
        .mount("/pleitos", pleitos::routes())
        .mount("/processos", processos::routes())
        .mount("/julgamentos", julgamentos::routes())
        .launch()
        .await?;
    Ok(())
}
