use serde::Serialize;
use std::convert::{TryFrom, TryInto};

use crate::database::{Client, Row};
use crate::error::ServerError;

use super::TipoCargo;

/// Candidatura política
#[derive(Debug, Serialize)]
pub struct Candidatura {
    pub candidato: String,
    pub vice_candidato: Option<String>,
    pub ano: i16,
    pub cargo_tipo: TipoCargo,
    pub cargo_local: String,
    pub numero: i32,
    pub partido: i16,
    pub votos: Option<i32>,
}

/// Converte da linha para o nosso cargo_tipo
impl TryFrom<Row> for Candidatura {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Candidatura, ServerError> {
        Ok(Candidatura {
            candidato: row.try_get("candidato")?,
            vice_candidato: row.try_get("vice_candidato")?,
            cargo_tipo: row.try_get("cargo_tipo")?,
            cargo_local: row.try_get("cargo_local")?,
            ano: row.try_get("ano")?,
            numero: row.try_get("numero")?,
            partido: row.try_get("partido")?,
            votos: row.try_get("votos")?,
        })
    }
}

impl Candidatura {
    /// Obtém uma candidatura, dado candidato e ano
    pub async fn obter(db: &Client, candidato: &str, ano: i16) -> Result<Candidatura, ServerError> {
        db.query_one(
            "SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido, votos
            FROM candidatura
            WHERE candidato = $1 AND ano = $2",
            &[&candidato, &ano],
        )
        .await?
        .try_into()
    }
    /// Obtém uma candidatura, dado vice candidato e ano
    pub async fn obter_do_vice(
        db: &Client,
        vice_candidato: &str,
        ano: i16,
    ) -> Result<Candidatura, ServerError> {
        db.query_one(
            "SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido, votos
            FROM candidatura
            WHERE vice_candidato = $1 AND ano = $2",
            &[&vice_candidato, &ano],
        )
        .await?
        .try_into()
    }
    /// Obtém uma candidatura, dado número, (tipo e local do) cargo, e ano
    pub async fn obter_do_numero(
        db: &Client,
        numero: i32,
        cargo_tipo: &TipoCargo,
        cargo_local: &str,
        ano: i16,
    ) -> Result<Candidatura, ServerError> {
        db.query_one(
            "SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido, votos
            FROM candidatura
            WHERE cargo_tipo = $1 AND cargo_local = $2 AND numero = $3 AND ano = $4",
            &[&cargo_tipo, &cargo_local, &numero, &ano],
        )
        .await?
        .try_into()
    }

    /// Lista as candidaturas, com filtros opcionais
    pub async fn listar(
        db: &Client,
        filtro: CandidaturaFiltro,
    ) -> Result<Vec<Candidatura>, ServerError> {
        db.query(
            "
            SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido, votos
            FROM candidatura
            WHERE
                ($1::VARCHAR IS NULL OR candidato = $1) AND
                ($2::VARCHAR IS NULL OR vice_candidato = $2) AND
                ($3::SMALLINT IS NULL OR ano = $3) AND
                ($4::tipo_cargo IS NULL OR cargo_tipo = $4) AND
                ($5::VARCHAR IS NULL OR cargo_local ILIKE $5) AND
                ($6::INTEGER IS NULL OR numero = $6) AND
                ($7::SMALLINT IS NULL OR partido = $7) AND
                ($8::INTEGER IS NULL OR votos >= $8) AND
                ($9::INTEGER IS NULL OR votos <= $9)
            ",
            &[
                &filtro.candidato,
                &filtro.vice_candidato,
                &filtro.ano,
                &filtro.cargo_tipo,
                &filtro.cargo_local,
                &filtro.numero,
                &filtro.partido,
                &filtro.min_votos,
                &filtro.max_votos,
            ],
        )
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }
    /// Cria um filtro para o metodo listar, pode ser manipulado usando os metodos dele
    pub fn filtro() -> CandidaturaFiltro {
        CandidaturaFiltro::default()
    }
}

/// Filtro de listagem de candidaturas
/// Funciona como um builder
#[derive(Default)]
pub struct CandidaturaFiltro {
    candidato: Option<String>,
    vice_candidato: Option<String>,
    ano: Option<i16>,
    cargo_tipo: Option<TipoCargo>,
    cargo_local: Option<String>,
    numero: Option<i32>,
    partido: Option<i16>,
    min_votos: Option<i32>,
    max_votos: Option<i32>,
}

impl CandidaturaFiltro {
    pub fn candidato(self, candidato: &str) -> Self {
        Self {
            candidato: Some(candidato.into()),
            ..self
        }
    }
    pub fn vice_candidato(self, vice_candidato: &str) -> Self {
        Self {
            vice_candidato: Some(vice_candidato.into()),
            ..self
        }
    }
    pub fn ano(self, ano: i16) -> Self {
        Self {
            ano: Some(ano),
            ..self
        }
    }
    pub fn cargo_tipo(self, cargo_tipo: TipoCargo) -> Self {
        Self {
            cargo_tipo: Some(cargo_tipo),
            ..self
        }
    }
    pub fn cargo_local(self, cargo_local: &str) -> Self {
        Self {
            cargo_local: Some(cargo_local.into()),
            ..self
        }
    }
    pub fn numero(self, numero: i32) -> Self {
        Self {
            numero: Some(numero),
            ..self
        }
    }
    pub fn partido(self, partido: i16) -> Self {
        Self {
            partido: Some(partido),
            ..self
        }
    }
    pub fn min_votos(self, min_votos: i32) -> Self {
        Self {
            min_votos: Some(min_votos),
            ..self
        }
    }
    pub fn max_votos(self, max_votos: i32) -> Self {
        Self {
            max_votos: Some(max_votos),
            ..self
        }
    }
}
