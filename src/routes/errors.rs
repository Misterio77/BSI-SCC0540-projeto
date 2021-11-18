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
