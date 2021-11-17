use crate::error::ServerError;
/// Rotas para pegar possíveis erros (como 404, por exemplo)
use rocket::{catch, catchers, http::Status, Catcher};

#[catch(404)]
fn not_found() -> ServerError {
    ServerError::builder()
        .code(Status::NotFound)
        .message("Rota não encontrada")
        .build()
}

#[catch(500)]
fn internal_server_error() -> ServerError {
    ServerError::builder()
        .code(Status::InternalServerError)
        .message("Erro inesperado ocorreu")
        .build()
}

pub fn catchers() -> Vec<Catcher> {
    catchers![not_found, internal_server_error]
}
