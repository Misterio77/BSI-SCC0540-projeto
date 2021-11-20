pub mod apoios;
pub mod candidaturas;
pub mod cargos;
pub mod doacoes;
pub mod individuos;
pub mod julgamentos;
pub mod partidos;
pub mod pleitos;
pub mod processos;

pub mod home {
    /// Página inicial do site
    use rocket::{get, request::FlashMessage, routes, Route};
    use rocket_dyn_templates::{context, Template};
    use crate::assets::{Assets, Asset};

    #[get("/")]
    fn index(flash: Option<FlashMessage<'_>>) -> Template {
        Template::render("base", context! {flash})
    }

    #[get("/assets/style.css")]
    async fn style_css(assets: &Assets) -> Option<Asset> {
        assets.open("style.css").await.ok()
    }

    pub fn routes() -> Vec<Route> {
        routes![index, style_css]
    }
}

pub mod errors {
    use crate::error::ServerError;
    /// Rotas para pegar possíveis erros (como 404, por exemplo)
    use rocket::{catch, catchers, http::Status, Catcher, Request};

    #[catch(404)]
    fn not_found() -> ServerError {
        ServerError::builder()
            .code(Status::NotFound)
            .message("Rota não encontrada")
            .build()
    }

    #[catch(503)]
    fn service_unavailable() -> ServerError {
        ServerError::builder()
            .code(Status::ServiceUnavailable)
            .message("A aplicação se encontra temporariamente indisponível")
            .build()
    }

    #[catch(default)]
    fn unknown_error(status: Status, _: &Request) -> ServerError {
        ServerError::builder()
            .code(status)
            .message("Um erro inesperado ocorreu")
            .build()
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![not_found, service_unavailable, unknown_error]
    }
}
