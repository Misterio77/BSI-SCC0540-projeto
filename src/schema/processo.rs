use chrono::NaiveDate;
use serde::Serialize;
use std::convert::{TryInto, TryFrom};

use crate::database::{Client, Row};
use crate::error::{Result, ServerError};

/// Processo judicial
#[derive(Debug, Serialize)]
pub struct Processo {
    pub id: i32,
    pub reu: String,
    pub crime: String,
    pub julgado: bool,
    pub data_julgamento: Option<NaiveDate>,
    pub procedente: Option<bool>,
    pub pena: Option<i16>,
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Processo {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Processo> {
        Ok(Processo {
            id: row.try_get("id")?,
            reu: row.try_get("reu")?,
            crime: row.try_get("crime")?,
            julgado: row.try_get("julgado")?,
            data_julgamento: row.try_get("data_julgamento")?,
            procedente: row.try_get("procedente")?,
            pena: row.try_get("pena")?,
        })
    }
}

impl Processo {
    /// Obtém um processo, dado seu id
    pub async fn obter(db: &Client, id: i32) -> Result<Processo> {
        db.query_one(
            "
            SELECT id, reu, crime, julgado, data_julgamento, procedente, pena
            FROM processo
            WHERE id = $1",
            &[&id]
        )
        .await?
        .try_into()
    }

    /// Lista os processos, com filtros opcionais
    pub async fn listar(db: &Client, filtro: ProcessoFiltro) -> Result<Vec<Processo>> {
        db.query(
            "
            SELECT id, reu, crime, julgado, data_julgamento, procedente, pena
            FROM processo
            WHERE
                ($1::INTEGER IS NULL OR id = $1) AND
                ($2::VARCHAR IS NULL OR reu ILIKE '%$2%') AND
                ($3::VARCHAR IS NULL OR crime ILIKE '%$3%') AND
                ($4::BOOLEAN IS NULL OR julgado = $4) AND
                ($5::DATE IS NULL OR data_julgamento = $5) AND
                ($6::BOOLEAN IS NULL OR procedente = $6) AND
                ($7::VARCHAR IS NULL OR pena ILIKE '%$7%')",
            &[
                &filtro.id,
                &filtro.reu,
                &filtro.crime,
                &filtro.julgado,
                &filtro.data_julgamento,
                &filtro.procedente,
                &filtro.pena,
            ],
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
    julgado: Option<bool>,
    data_julgamento: Option<NaiveDate>,
    procedente: Option<bool>,
    pena: Option<i16>,
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
    pub fn julgado(self, julgado: bool) -> Self {
        Self {
            julgado: Some(julgado),
            ..self
        }
    }
    pub fn data_julgamento(self, data_julgamento: &NaiveDate) -> Self {
        Self {
            data_julgamento: Some(data_julgamento.clone()),
            ..self
        }
    }
    pub fn procedente(self, procedente: bool) -> Self {
        Self {
            procedente: Some(procedente),
            ..self
        }
    }
    pub fn pena(self, pena: i16) -> Self {
        Self {
            pena: Some(pena),
            ..self
        }
    }
}
