use serde::Serialize;
use std::convert::{TryInto, TryFrom};

use crate::database::{Client, Row};
use crate::error::{Result, ServerError};

use super::Candidatura;

/// Partido político
#[derive(Debug, Serialize)]
pub struct Partido {
    numero: i16,
    nome: String,
    programa: String,
}

impl Partido {
    /// Obtém um partido, dado seu número
    pub async fn obter(db: &Client, numero: i16) -> Result<Partido> {
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
    pub async fn obter_do_nome(db: &Client, nome: &str) -> Result<Partido> {
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

    /// Lista os partidos (esse não tem filtros, pois todos os atributos são efetivamente únicos)
    pub async fn listar(db: &Client) -> Result<Vec<Partido>> {
        db.query(
            "
            SELECT numero, nome, programa
            FROM partido",
            &[],
        )
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }

    // === Obter entidades relacionadas ===
    /// Retorna as candidaturas filiadas ao partido
    pub async fn candidaturas(&self, db: &Client) -> Result<Vec<Candidatura>> {
        Candidatura::listar(db, Candidatura::filtro().partido(self.numero))
        .await
    }
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Partido {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Partido> {
        Ok(Partido {
            numero: row.try_get("numero")?,
            nome: row.try_get("nome")?,
            programa: row.try_get("programa")?,
        })
    }
}
