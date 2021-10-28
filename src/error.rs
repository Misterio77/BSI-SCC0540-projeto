use rocket::http::Status;

use std::error::Error as StdError;
use std::fmt;
use std::result::Result as StdResult;

/// Alias para facilitar o uso de Result
pub type Result<T> = StdResult<T, ServerError>;

/// Possível erro do nosso servidor
#[derive(Debug)]
pub struct ServerError {
    /// Código de status http
    code: Status,
    /// Erro originário, se existir
    source: Option<Box<dyn StdError + Sync + Send>>,
    /// Mensagem amigável, se existir
    message: Option<String>,
}

impl ServerError {
    /// Retorna um builder com os campos padrão
    pub fn builder() -> ServerErrorBuilder {
        ServerError::default().edit()
    }
    /// Transforma um erro existente de volta no builder
    pub fn edit(self) -> ServerErrorBuilder {
        ServerErrorBuilder { inner: self }
    }
    /// Converte usando `From` e daí transforma em builder
    /// (facilita adicionar contexto à outros erros)
    pub fn builder_from<T: Into<ServerError>>(source: T) -> ServerErrorBuilder {
        let error = source.into();
        error.edit()
    }
}

/// Construtor padrão
impl Default for ServerError {
    fn default() -> Self {
        ServerError {
            code: Status::InternalServerError,
            source: None,
            message: None,
        }
    }
}

/// Implementação display (pretty print)
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code.reason_lossy())?;
        if let Some(message) = &self.message {
            write!(f, " ({})", message)?;
        };
        if let Some(source) = &self.source {
            write!(f, ": {}", source)?;
        };
        Ok(())
    }
}

/// Implementação erro
impl StdError for ServerError {
    /// Obter o erro originário
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|s| &**s as _)
    }
}

/// Converte erro do rocket
impl From<rocket::error::Error> for ServerError {
    fn from(e: rocket::error::Error) -> Self {
        ServerError::builder()
            .code(Status::ServiceUnavailable)
            .source(Box::new(e))
            .message("Não foi possível iniciar o servidor")
            .build()
    }
}
/// Converte erro do banco de dados
impl From<rocket_db_pools::deadpool_postgres::tokio_postgres::Error> for ServerError {
    fn from(e: rocket_db_pools::deadpool_postgres::tokio_postgres::Error) -> Self {
        ServerError::builder()
            .source(Box::new(e))
            .message("Não foi possível completar operação na base de dados")
            .build()
    }
}

/// Builder para o ServerError, adiciona ergonomia
pub struct ServerErrorBuilder {
    inner: ServerError,
}

impl ServerErrorBuilder {
    /// Finaliza o builder e constrói o erro
    pub fn build(self) -> ServerError {
        self.inner
    }

    /// Adiciona código de erro ao builder
    pub fn code(self, code: Status) -> ServerErrorBuilder {
        ServerErrorBuilder {
            inner: ServerError { code, ..self.inner },
        }
    }
    /// Adiciona fonte do erro ao builder
    pub fn source(self, source: Box<dyn StdError + Sync + Send>) -> ServerErrorBuilder {
        ServerErrorBuilder {
            inner: ServerError {
                source: Some(source),
                ..self.inner
            },
        }
    }
    /// Adiciona mensagem de erro ao builder
    pub fn message(self, message: &str) -> ServerErrorBuilder {
        ServerErrorBuilder {
            inner: ServerError {
                message: Some(message.into()),
                ..self.inner
            },
        }
    }
}
