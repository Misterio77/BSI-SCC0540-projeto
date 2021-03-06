use std::error::Error as StdError;
use std::fmt;

use rocket::{
    http::Status,
    response::{Flash, Redirect},
};
use rocket_dyn_templates::Template;
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

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

    /// Converte em um Flash<Redirect>, que é um redirecionamento contendo um cookie para
    /// exibir uma pequena mensagem de erro na tela.
    pub fn flash_redirect(&self, url: &str) -> Flash<Redirect> {
        let message = self.message.as_deref().unwrap_or("Erro desconhecido.");
        Flash::error(Redirect::to(url.to_string()), message)
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
/// Converte erro de parse no tipo de cargo
impl From<strum::ParseError> for ServerError {
    fn from(e: strum::ParseError) -> Self {
        ServerError::builder()
            .code(Status::BadRequest)
            .source(Box::new(e))
            .message("Tipo de cargo inválido")
            .build()
    }
}
/// Converte erro do banco de dados
impl From<rocket_db_pools::deadpool_postgres::tokio_postgres::Error> for ServerError {
    fn from(e: rocket_db_pools::deadpool_postgres::tokio_postgres::Error) -> Self {
        let initial_message = match e.as_db_error() {
            // Caso seja erro levantado pelo postgres, fazer downcast para pegar mensagem detalhada
            Some(db_e) => {
                // Pegar a mensagem dele
                db_e.message().into()
            }
            // Caso seja erro levantado pela biblioteca
            None => {
                // Pegar a representação textual dele
                format!("{}", e)
            }
        };

        // Obter mensagem e status bonitinhos
        let (message, status) = pretty_db_error(&initial_message);

        ServerError::builder()
            .message(&message)
            .code(status)
            .source(Box::new(e))
            .build()
    }
}

/// Baseado numa mensagem inicial de erro, deixar traduzida e bonitinha
fn pretty_db_error(initial: &str) -> (String, Status) {
    match initial {
        "query returned an unexpected number of rows" => (
            "Fileira não encontrada na base de dados.".into(),
            Status::NotFound,
        ),
        m => (
            format!("Erro na base de dados: {}", m),
            Status::InternalServerError,
        ),
    }
}

/// Permite que o erro seja diretamente retornado de uma rota, ou seja, esse traço serve para
/// transformar o erro (e consequentemente, um result que pode conter um erro) diretamente em
/// uma resposta HTTP
impl<'r> rocket::response::Responder<'r, 'static> for ServerError {
    fn respond_to(
        self,
        req: &'r rocket::request::Request<'_>,
    ) -> rocket::response::Result<'static> {
        let media_type = req.accept().map(|a| a.preferred().media_type());

        let mut response = rocket::response::Response::build();
        response.status(self.code);

        if media_type == Some(&rocket::http::MediaType::JSON) {
            let json = rocket::serde::json::Json(self);

            response.join(json.respond_to(req)?)
        } else {
            let template = Template::render("error", self);

            response.join(template.respond_to(req)?)
        };

        response.ok()
    }
}

impl Serialize for ServerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ServerError", 2)?;
        state.serialize_field("code", &format!("{}", &self.code))?;
        state.serialize_field("description", &self.message)?;
        state.serialize_field(
            "reason",
            &self
                .source
                .as_ref()
                .map(|s| format!("{:?}", s).replace("\"", "'").replace("\\'", "'")),
        )?;
        state.end()
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
