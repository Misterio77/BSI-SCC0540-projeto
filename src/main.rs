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
        assets, home, errors,
        candidaturas, cargos, doacoes, individuos, partidos, pleitos, processos, julgamentos
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
        .mount("/candidaturas", routes![candidaturas::get, candidaturas::list])
        .mount("/cargos", routes![cargos::get])
        .mount("/doacoes", routes![doacoes::get])
        .mount("/individuos", routes![individuos::get])
        .mount("/partidos", routes![partidos::get])
        .mount("/pleitos", routes![pleitos::get])
        .mount("/processos", routes![processos::get])
        .mount("/julgamentos", routes![julgamentos::get])
        .launch()
        .await?;
    Ok(())
}
