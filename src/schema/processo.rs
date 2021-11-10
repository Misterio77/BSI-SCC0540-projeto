use serde::Serialize;
use std::convert::{TryFrom, TryInto};

use crate::database::{Client, Row};
use crate::error::ServerError;

/// Processo judicial
#[derive(Debug, Serialize)]
pub struct Processo {
    pub id: i32,
    pub reu: String,
    pub crime: String,
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Processo {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Processo, ServerError> {
        Ok(Processo {
            id: row.try_get("id")?,
            reu: row.try_get("reu")?,
            crime: row.try_get("crime")?,
        })
    }
}

impl Processo {
    /// Obtém um processo, dado seu id
    pub async fn obter(db: &Client, id: i32) -> Result<Processo, ServerError> {
        db.query_one(
            "
            SELECT id, reu, crime
            FROM processo
            WHERE id = $1",
            &[&id],
        )
        .await?
        .try_into()
    }

    /// Lista os processos, com filtros opcionais
    pub async fn listar(db: &Client, filtro: ProcessoFiltro) -> Result<Vec<Processo>, ServerError> {
        db.query(
            "
            SELECT id, reu, crime
            FROM processo
            WHERE
                ($1::INTEGER IS NULL OR id = $1) AND
                ($2::VARCHAR IS NULL OR reu ILIKE '%$2%') AND
                ($3::VARCHAR IS NULL OR crime ILIKE '%$3%')",
            &[&filtro.id, &filtro.reu, &filtro.crime],
        )
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }
}

/// Filtro de listagem de processo
/// Funciona como um builder
#[derive(Default)]
pub struct ProcessoFiltro {
    id: Option<i32>,
    reu: Option<String>,
    crime: Option<String>,
}

impl ProcessoFiltro {
    pub fn id(self, id: i32) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }
    pub fn reu(self, reu: &str) -> Self {
        Self {
            reu: Some(reu.into()),
            ..self
        }
    }
    pub fn crime(self, crime: &str) -> Self {
        Self {
            crime: Some(crime.into()),
            ..self
        }
    }
}
