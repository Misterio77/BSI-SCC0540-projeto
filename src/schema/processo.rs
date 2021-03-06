use rocket::form::{FromForm, FromFormField};
use serde::Serialize;
use std::convert::{TryFrom, TryInto};
use strum::Display;

use crate::database::{Client, Row};
use crate::error::ServerError;

/// Processo judicial
#[derive(Clone, Serialize)]
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

    /// Deletar pleito
    pub async fn remover(self, db: &Client) -> Result<(), ServerError> {
        db.execute(
            "DELETE FROM processo
            WHERE id = $1",
            &[&self.id],
        )
        .await?;
        Ok(())
    }

    /// Lista os processos, com filtros opcionais
    pub async fn listar(
        db: &Client,
        filtro: ProcessoFiltro,
        pagina: u16,
        limite: u16,
    ) -> Result<Vec<Processo>, ServerError> {
        let filtro = filtro.cleanup();

        let query = format!(
            "SELECT id, reu, crime
            FROM processo
            WHERE
                ($1::INTEGER IS NULL OR id        = $1) AND
                ($2::VARCHAR IS NULL OR reu       = $2) AND
                ($3::VARCHAR IS NULL OR crime ILIKE $3)
            {} LIMIT $4 OFFSET $5",
            // Caso tenha ordenação, adicionar ORDER BY nome
            if let Some(ord) = filtro.ordenacao {
                format!(
                    "ORDER BY {} {}",
                    ord,
                    if filtro.ordenacao_desc { "DESC" } else { "" }
                )
            } else {
                "".to_string()
            },
        );
        db.query(
            &query,
            &[
                &filtro.id,
                &filtro.reu,
                &filtro.crime,
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

/// Filtro de listagem de processo
#[derive(Clone, Serialize, FromForm)]
pub struct ProcessoFiltro {
    id: Option<i32>,
    reu: Option<String>,
    crime: Option<String>,
    ordenacao: Option<ProcessoOrdenacao>,
    ordenacao_desc: bool,
}
impl ProcessoFiltro {
    pub fn cleanup(self) -> Self {
        Self {
            reu: self.reu.filter(|s| !s.is_empty()),
            crime: self
                .crime
                .filter(|s| !s.is_empty())
                .map(|s| format!("%{}%", s)),
            ..self
        }
    }
}

#[derive(Clone, Debug, Copy, Serialize, FromFormField, Display)]
#[strum(serialize_all = "snake_case")]
enum ProcessoOrdenacao {
    Id,
    Reu,
    Crime,
}
