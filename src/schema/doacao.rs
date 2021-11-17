use rust_decimal::Decimal;
use serde::Serialize;
use std::convert::{TryFrom, TryInto};

use crate::database::{Client, Row};
use crate::error::ServerError;

/// Doação para candidatura
#[derive(Debug, Serialize)]
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
    pub async fn listar(db: &Client, filtro: DoacaoFiltro) -> Result<Vec<Doacao>, ServerError> {
        db.query(
            "
            SELECT id, valor, doador, candidato, ano
            FROM doacao
            WHERE
                ($1::INTEGER IS NULL OR id = $1) AND
                ($2::NUMERIC IS NULL OR valor = $2) AND
                ($3::VARCHAR IS NULL OR doador = $3) AND
                ($4::VARCHAR IS NULL OR candidato = $4) AND
                ($5::SMALLINT IS NULL OR ano >= $5) AND
                ($6::SMALLINT IS NULL OR ano <= $6)
            ",
            &[
                &filtro.id,
                &filtro.valor,
                &filtro.doador,
                &filtro.candidato,
                &filtro.min_ano,
                &filtro.max_ano,
            ],
        )
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }

    /// Cria um filtro para o metodo listar, pode ser manipulado usando os metodos dele
    pub fn filtro() -> DoacaoFiltro {
        DoacaoFiltro::default()
    }
}

/// Filtro de listagem de doações
#[derive(Default, Serialize, Debug)]
pub struct DoacaoFiltro {
    pub id: Option<i32>,
    pub valor: Option<Decimal>,
    pub doador: Option<String>,
    pub candidato: Option<String>,
    pub min_ano: Option<i16>,
    pub max_ano: Option<i16>,
}
