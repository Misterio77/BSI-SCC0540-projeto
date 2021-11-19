use rocket::form::FromForm;
use serde::Serialize;
use std::convert::{TryFrom, TryInto};

use crate::database::{Client, Row};
use crate::error::ServerError;

/// Partido político
#[derive(Clone, Serialize)]
pub struct Partido {
    pub numero: i16,
    pub nome: String,
    pub programa: String,
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Partido {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Partido, ServerError> {
        Ok(Partido {
            numero: row.try_get("numero")?,
            nome: row.try_get("nome")?,
            programa: row.try_get("programa")?,
        })
    }
}

impl Partido {
    /// Obtém um partido, dado seu número
    pub async fn obter(db: &Client, numero: i16) -> Result<Partido, ServerError> {
        db.query_one(
            "
            SELECT numero, nome, programa
            FROM partido
            WHERE numero = $1",
            &[&numero],
        )
        .await?
        .try_into()
    }
    /// Obtém um partido, dado seu nome
    pub async fn obter_do_nome(db: &Client, nome: &str) -> Result<Partido, ServerError> {
        db.query_one(
            "
            SELECT numero, nome, programa
            FROM partido
            WHERE nome = $1",
            &[&nome],
        )
        .await?
        .try_into()
    }

    /// Lista os partidos
    pub async fn listar(
        db: &Client,
        filtro: PartidoFiltro,
        pagina: u16,
        limite: u16,
    ) -> Result<Vec<Partido>, ServerError> {
        let filtro = filtro.cleanup();
        db.query(
            "
            SELECT numero, nome, programa
            FROM partido
            WHERE
                ($1::SMALLINT IS NULL OR numero   = $1) AND
                ($2::VARCHAR  IS NULL OR nome ILIKE $2)
            LIMIT $3 OFFSET $4",
            &[
                &filtro.numero,
                &filtro.nome,
                &(limite as i64),
                &(((pagina - 1) as i64) * (limite as i64)),
            ],
        )
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }
}

/// Filtro de listagem de partido
#[derive(Clone, Serialize, FromForm)]
pub struct PartidoFiltro {
    numero: Option<i16>,
    nome: Option<String>,
}
impl PartidoFiltro {
    pub fn cleanup(self) -> Self {
        Self {
            nome: self
                .nome
                .filter(|s| !s.is_empty())
                .map(|s| format!("%{}%", s)),
            ..self
        }
    }
}
