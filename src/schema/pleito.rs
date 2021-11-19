use rocket::form::FromForm;
use serde::Serialize;
use std::convert::{TryFrom, TryInto};

use crate::database::{Client, Row};
use crate::error::ServerError;

/// Pleito de um pleito
#[derive(Clone, Serialize)]
pub struct Pleito {
    pub candidato: String,
    pub ano: i16,
    pub turno: i16,
    pub votos: i32,
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Pleito {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Pleito, ServerError> {
        Ok(Pleito {
            candidato: row.try_get("candidato")?,
            ano: row.try_get("ano")?,
            turno: row.try_get("turno")?,
            votos: row.try_get("votos")?,
        })
    }
}

impl Pleito {
    /// Obtém um pleito, dado o candidato, ano, e o número do turno
    pub async fn obter(
        db: &Client,
        candidato: &str,
        ano: i16,
        turno: i16,
    ) -> Result<Pleito, ServerError> {
        db.query_one(
            "
            SELECT candidato, ano, turno, votos
            FROM pleito
            WHERE candidato = $1 AND ano = $2 AND turno = $3",
            &[&candidato, &ano, &turno],
        )
        .await?
        .try_into()
    }

    /// Lista os pleitos, com filtros opcionais
    pub async fn listar(
        db: &Client,
        filtro: PleitoFiltro,
        pagina: u16,
        limite: u16,
    ) -> Result<Vec<Pleito>, ServerError> {
        let filtro = filtro.cleanup();
        db.query(
            "
            SELECT candidato, ano, turno, votos
            FROM pleito
            WHERE
                ($1::VARCHAR  IS NULL OR candidato = $1) AND
                ($2::SMALLINT IS NULL OR ano       = $2) AND
                ($3::SMALLINT IS NULL OR turno     = $3) AND
                ($4::INTEGER  IS NULL OR votos    >= $4) AND
                ($5::INTEGER  IS NULL OR votos    <= $5)
            LIMIT $6 OFFSET $7",
            &[
                &filtro.candidato,
                &filtro.ano,
                &filtro.turno,
                &filtro.min_votos,
                &filtro.max_votos,
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

/// Filtro de listagem de pleito
#[derive(Clone, Serialize, FromForm)]
pub struct PleitoFiltro {
    candidato: Option<String>,
    ano: Option<i16>,
    turno: Option<i16>,
    min_votos: Option<i32>,
    max_votos: Option<i32>,
}
impl PleitoFiltro {
    pub fn cleanup(self) -> Self {
        Self {
            candidato: self.candidato.filter(|s| !s.is_empty()),
            ..self
        }
    }
}
