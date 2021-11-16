use chrono::NaiveDate;
use serde::Serialize;
use std::convert::{TryFrom, TryInto};

use crate::database::{Client, Row};
use crate::error::ServerError;

/// Julgamento de um julgamento
#[derive(Debug, Serialize)]
pub struct Julgamento {
    pub processo: i32,
    pub instancia: String,
    pub data: NaiveDate,
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

    /// Lista os julgamentos, com filtros opcionais
    pub async fn listar(
        db: &Client,
        filtro: JulgamentoFiltro,
    ) -> Result<Vec<Julgamento>, ServerError> {
        db.query(
            "
            SELECT processo, instancia, data, procedente
            FROM julgamento
            WHERE
                ($1::INTEGER IS NULL OR processo = $1) AND
                ($2::VARCHAR IS NULL OR instancia ILIKE '%$2%') AND
                ($3::DATE IS NULL OR data = $3) AND
                ($4::BOOLEAN IS NULL OR procedente = $4)",
            &[
                &filtro.processo,
                &filtro.instancia,
                &filtro.data,
                &filtro.procedente,
            ],
        )
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }
}

/// Filtro de listagem de julgamento
/// Funciona como um builder
#[derive(Default)]
pub struct JulgamentoFiltro {
    processo: Option<i32>,
    instancia: Option<String>,
    data: Option<NaiveDate>,
    procedente: Option<bool>,
}

impl JulgamentoFiltro {
    pub fn processo(self, processo: i32) -> Self {
        Self {
            processo: Some(processo),
            ..self
        }
    }
    pub fn instancia(self, instancia: &str) -> Self {
        Self {
            instancia: Some(instancia.into()),
            ..self
        }
    }
    pub fn data(self, data: NaiveDate) -> Self {
        Self {
            data: Some(data),
            ..self
        }
    }
    pub fn procedente(self, procedente: bool) -> Self {
        Self {
            procedente: Some(procedente),
            ..self
        }
    }
}
