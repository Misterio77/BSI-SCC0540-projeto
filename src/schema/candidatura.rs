use rocket::form::FromForm;
use serde::Serialize;
use std::convert::{TryFrom, TryInto};

use crate::database::{Client, Row};
use crate::error::ServerError;

use super::TipoCargo;

/// Candidatura política
#[derive(Clone, Serialize)]
pub struct Candidatura {
    pub candidato: String,
    pub vice_candidato: Option<String>,
    pub ano: i16,
    pub cargo_tipo: TipoCargo,
    pub cargo_local: String,
    pub numero: i32,
    pub partido: i16,
}

/// Converte da linha para o nosso tipo
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
        })
    }
}

impl Candidatura {
    /// Obtém uma candidatura, dado candidato e ano
    pub async fn obter(db: &Client, candidato: &str, ano: i16) -> Result<Candidatura, ServerError> {
        db.query_one(
            "SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido
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
            "SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido
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
            "SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido
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
        pagina: u16,
        limite: u16,
    ) -> Result<Vec<Candidatura>, ServerError> {
        let filtro = filtro.cleanup();

        // Mostrar apenas os eleitos?
        let query = if filtro.eleitos {
            // Query especial, que usa funções window para pegar apenas os top candidatos, com o
            // número de cadeiras do cargo como limite
            "
            SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido FROM (
                SELECT
                    c.candidato, c.vice_candidato, c.ano, c.cargo_tipo, c.cargo_local, c.numero, c.partido,
                    row_number() OVER(
                        PARTITION BY c.cargo_tipo, c.cargo_local, c.ano
                        ORDER BY p.turno DESC, p.votos DESC
                    ) AS rownum
                FROM
                    candidatura c
                INNER JOIN pleito p
                    ON p.candidato = c.candidato
                    AND p.ano = c.ano
            ) AS eleicao
            INNER JOIN cargo
                ON cargo.local = eleicao.cargo_local
                AND cargo.tipo = eleicao.cargo_tipo
            WHERE
                rownum <= cargo.cadeiras AND
                ($1::VARCHAR    IS NULL OR candidato       = $1) AND
                ($2::VARCHAR    IS NULL OR vice_candidato  = $2) AND
                ($3::SMALLINT   IS NULL OR ano             = $3) AND
                ($4::tipo_cargo IS NULL OR cargo_tipo      = $4) AND
                ($5::VARCHAR    IS NULL OR cargo_local ILIKE $5) AND
                ($6::INTEGER    IS NULL OR numero          = $6) AND
                ($7::SMALLINT   IS NULL OR partido         = $7)
            LIMIT $8 OFFSET $9"
        } else {
            // Query usual
            "
            SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido
            FROM candidatura
            WHERE
                ($1::VARCHAR    IS NULL OR candidato       = $1) AND
                ($2::VARCHAR    IS NULL OR vice_candidato  = $2) AND
                ($3::SMALLINT   IS NULL OR ano             = $3) AND
                ($4::tipo_cargo IS NULL OR cargo_tipo      = $4) AND
                ($5::VARCHAR    IS NULL OR cargo_local ILIKE $5) AND
                ($6::INTEGER    IS NULL OR numero          = $6) AND
                ($7::SMALLINT   IS NULL OR partido         = $7)
            LIMIT $8 OFFSET $9"
        };

        db.query(
            query,
            &[
                &filtro.candidato,
                &filtro.vice_candidato,
                &filtro.ano,
                &filtro.cargo_tipo,
                &filtro.cargo_local,
                &filtro.numero,
                &filtro.partido,
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

/// Filtro de listagem de candidaturas
#[derive(Clone, Serialize, FromForm)]
pub struct CandidaturaFiltro {
    pub candidato: Option<String>,
    pub vice_candidato: Option<String>,
    pub ano: Option<i16>,
    pub cargo_tipo: Option<TipoCargo>,
    pub cargo_local: Option<String>,
    pub numero: Option<i32>,
    pub partido: Option<i16>,
    pub eleitos: bool,
}
impl CandidaturaFiltro {
    pub fn cleanup(self) -> Self {
        Self {
            candidato: self.candidato.filter(|s| !s.is_empty()),
            vice_candidato: self.vice_candidato.filter(|s| !s.is_empty()),
            cargo_local: self
                .cargo_local
                .filter(|s| !s.is_empty())
                .map(|s| format!("%{}%", s)),
            ..self
        }
    }
}
