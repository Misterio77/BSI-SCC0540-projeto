use rocket::form::FromForm;
use rust_decimal::Decimal;
use serde::Serialize;
use std::convert::{TryFrom, TryInto};

use crate::database::{Client, Row};
use crate::error::ServerError;

/// Doação para candidatura
#[derive(Clone, Serialize)]
pub struct Doacao {
    pub id: i32,
    pub valor: Decimal,
    pub doador: String,
    pub candidato: String,
    pub ano: i16,
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Doacao {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Doacao, ServerError> {
        Ok(Doacao {
            id: row.try_get("id")?,
            valor: row.try_get("valor")?,
            doador: row.try_get("doador")?,
            candidato: row.try_get("candidato")?,
            ano: row.try_get("ano")?,
        })
    }
}

impl Doacao {
    /// Obtém doação, dado id
    pub async fn obter(db: &Client, id: i32) -> Result<Doacao, ServerError> {
        db.query_one(
            "
            SELECT id, valor, doador, candidato, ano
            FROM doacao
            WHERE id = $1",
            &[&id],
        )
        .await?
        .try_into()
    }

    /// Lista as doações, com filtros opcionais
    pub async fn listar(
        db: &Client,
        filtro: DoacaoFiltro,
        pagina: u16,
        limite: u16,
    ) -> Result<Vec<Doacao>, ServerError> {
        let filtro = filtro.cleanup();
        db.query(
            "
            SELECT id, valor, doador, candidato, ano
            FROM doacao
            WHERE
                ($1::INTEGER  IS NULL OR id        = $1) AND
                ($2::VARCHAR  IS NULL OR doador    = $2) AND
                ($3::VARCHAR  IS NULL OR candidato = $3) AND
                ($4::NUMERIC  IS NULL OR valor    >= $4) AND
                ($5::NUMERIC  IS NULL OR valor    <= $5) AND
                ($6::SMALLINT IS NULL OR ano       = $6)
            LIMIT $7 OFFSET $8",
            &[
                &filtro.id,
                &filtro.doador,
                &filtro.candidato,
                &filtro.min_valor,
                &filtro.max_valor,
                &filtro.ano,
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

/// Filtro de listagem de doações
#[derive(Clone, Serialize, FromForm)]
pub struct DoacaoFiltro {
    id: Option<i32>,
    min_valor: Option<Decimal>,
    max_valor: Option<Decimal>,
    doador: Option<String>,
    candidato: Option<String>,
    ano: Option<i16>,
}
impl DoacaoFiltro {
    pub fn cleanup(self) -> Self {
        Self {
            doador: self.doador.filter(|s| !s.is_empty()),
            candidato: self.candidato.filter(|s| !s.is_empty()),
            ..self
        }
    }
}
