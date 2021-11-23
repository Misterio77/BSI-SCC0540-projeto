use rocket_db_pools::Database as DatabaseTrait;
use rocket_dyn_templates::Template;

use projeto_bd::{
    // Assets estáticos
    assets::Assets,
    // Nosso banco de dados
    database::Database,
    // Nosso tipo personalizado de erro
    error::ServerError,
    // Reescrever deleções
    post_as_delete::PostAsDelete,
    // Rotas do servidor
    routes::{
        apoios, candidaturas, cargos, doacoes, individuos, julgamentos, partidos, pleitos,
        processos,
    },
    // Páginas de erro, e home
    routes::{errors, home},
    // Funções customs do tera
    tera::customize,
};

#[rocket::main]
async fn main() -> Result<(), ServerError> {
    rocket::build()
        // Middleware pra conexões de database
        .attach(Database::init())
        // Middleware pra gerir templates html
        .attach(Template::custom(customize))
        // Middleware para gerir assets (style.css)
        .attach(Assets::fairing())
        // Middleware pra reescrever POST de deleções em DELETE
        .attach(PostAsDelete)
        // Páginas de erro
        .register("/", errors::catchers())
        // Página inicial
        .mount("/", home::routes())
        // Rotas das entidades
        .mount("/apoios", apoios::routes())
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
