use rust_decimal::Decimal;
use serde::Serialize;
use std::convert::{TryInto, TryFrom};

use crate::database::{Client, Row};
use crate::error::{Result, ServerError};

/// Doação para candidatura
#[derive(Debug, Serialize)]
pub struct Doacao {
    /// Identificador da doação
    id: i32,
    /// CPF ou CNPJ do doador
    doador: String,
    /// Valor (decimal garante precisão esperada para dinheiro)
    /// (Equivale ao NUMERIC do postgres)
    valor: Decimal,
}

impl Doacao {
    /// Obtém doação, dado id
    pub async fn obter(db: &Client, id: i32) -> Result<Doacao> {
        db.query_one(
            "
            SELECT id, valor
            FROM doacao
            WHERE id = #1",
            &[&id],
        )
        .await?
        .try_into()
    }

    /*
    /// Lista as doações, com filtros opcionais
    pub fn listar(
        candidatura: Option<i16>,
        ano: Option<i16>,
        valor: Option<Decimal>,
    ) -> Result<Vec<Doacao>> {
    }

    // === Obter entidades relacionadas ===
    /// Retorna a candidatura cuja doação foi destinada
    pub fn candidatura(&self) -> Result<Candidatura> {}
    /// Retorna o indivíduo que fez a doação
    pub fn doador(&self) -> Result<Individuo> {}
    */
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Doacao {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Doacao> {
        Ok(Doacao {
            id: row.try_get("id")?,
            doador: row.try_get("doador")?,
            valor: row.try_get("valor")?,
        })
    }
}
