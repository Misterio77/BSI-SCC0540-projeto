use rocket_db_pools::Database as DatabaseTrait;
use rocket_dyn_templates::Template;

use projeto_bd::{
    // Nosso banco de dados
    database::Database,
    // Assets estáticos
    assets::Assets,
    // Nosso tipo personalizado de erro
    error::ServerError,
    // Páginas de erro, e home
    routes::{errors, home},
    // Rotas do servidor
    routes::{
        candidaturas, cargos, doacoes, individuos, julgamentos, partidos, pleitos, processos,
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
        .attach(Assets)
        // Páginas de erro
        .register("/", errors::catchers())
        // Página inicial
        .mount("/", home::routes())
        // Rotas das entidades
        .mount("/candidaturas", candidaturas::routes())
        .mount("/cargos", cargos::routes())
        .mount("/doacoes", doacoes::routes())
        .mount("/individuos", individuos::routes())
        .mount("/partidos", partidos::routes())
        .mount("/pleitos", pleitos::routes())
        .mount("/processos", processos::routes())
        .mount("/julgamentos", julgamentos::routes())
        // Inicializar
        .launch()
        .await?;
    Ok(())
}
