use rocket::form::FromForm;
use serde::Serialize;
use std::convert::{TryFrom, TryInto};
use time::Date;

use crate::database::{Client, Row};
use crate::error::ServerError;

/// Julgamento de um julgamento
#[derive(Clone, Serialize)]
pub struct Julgamento {
    pub processo: i32,
    pub instancia: String,
    pub data: Date,
    pub procedente: bool,
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Julgamento {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Julgamento, ServerError> {
        Ok(Julgamento {
            processo: row.try_get("processo")?,
            instancia: row.try_get("instancia")?,
            data: row.try_get("data")?,
            procedente: row.try_get("procedente")?,
        })
    }
}

impl Julgamento {
    /// Obtém um julgamento, dado o id do processo e a instância onde ocorreu
    pub async fn obter(
        db: &Client,
        processo: i32,
        instancia: &str,
    ) -> Result<Julgamento, ServerError> {
        db.query_one(
            "
            SELECT processo, instancia, data, procedente
            FROM julgamento
            WHERE processo = $1 AND instancia = $2",
            &[&processo, &instancia],
        )
        .await?
        .try_into()
    }

    /// Deletar julgamento
    pub async fn remover(self, db: &Client) -> Result<(), ServerError> {
        db.execute(
            "DELETE FROM julgamento
            WHERE processo = $1 AND instancia = $2",
            &[&self.processo, &self.instancia],
        )
        .await?;
        Ok(())
    }

    /// Lista os julgamentos, com filtros opcionais
    pub async fn listar(
        db: &Client,
        filtro: JulgamentoFiltro,
        pagina: u16,
        limite: u16,
    ) -> Result<Vec<Julgamento>, ServerError> {
        let filtro = filtro.cleanup();
        db.query(
            "
            SELECT processo, instancia, data, procedente
            FROM julgamento
            WHERE
                ($1::INTEGER IS NULL OR processo   = $1) AND
                ($2::VARCHAR IS NULL OR instancia  = $2) AND
                ($3::DATE    IS NULL OR data       = $3) AND
                ($4::BOOLEAN IS NULL OR procedente = $4)
            LIMIT $5 OFFSET $6",
            &[
                &filtro.processo,
                &filtro.instancia,
                &filtro.data,
                &filtro.procedente,
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

/// Filtro de listagem de julgamento
#[derive(Clone, Serialize, FromForm)]
pub struct JulgamentoFiltro {
    processo: Option<i32>,
    instancia: Option<String>,
    data: Option<Date>,
    procedente: Option<bool>,
}
impl JulgamentoFiltro {
    pub fn cleanup(self) -> Self {
        Self {
            instancia: self.instancia.filter(|s| !s.is_empty()),
            ..self
        }
    }
}
