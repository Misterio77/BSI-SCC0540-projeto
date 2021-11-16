/// Rotas para pegar possíveis erros (como 404, por exemplo)
use rocket::{catch, http::Status};
use crate::error::ServerError;

#[catch(404)]
pub fn not_found() -> ServerError {
    ServerError::builder()
        .code(Status::NotFound)
        .message("Rota não encontrada")
        .build()
}
