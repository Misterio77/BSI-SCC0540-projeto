use rocket::form::{FromForm, FromFormField};
use serde::Serialize;
use std::convert::{TryFrom, TryInto};
use strum::Display;

use crate::database::{Client, Row};
use crate::error::ServerError;

/// Apoiador de candidatura
#[derive(Clone, Serialize)]
pub struct Apoio {
    pub apoiador: String,
    pub candidato: String,
    pub ano: i16,
    pub funcao: String,
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Apoio {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Apoio, ServerError> {
        Ok(Apoio {
            apoiador: row.try_get("apoiador")?,
            candidato: row.try_get("candidato")?,
            ano: row.try_get("ano")?,
            funcao: row.try_get("funcao")?,
        })
    }
}

impl Apoio {
    /// Obtém apoio, dado apoiador, candidato, e ano
    pub async fn obter(db: &Client, apoiador: &str, ano: i16) -> Result<Apoio, ServerError> {
        db.query_one(
            "
            SELECT apoiador, candidato, ano, funcao
            FROM apoio
            WHERE apoiador = $1 AND ano = $2",
            &[&apoiador, &ano],
        )
        .await?
        .try_into()
    }
    /// Deletar apoio
    pub async fn remover(self, db: &Client) -> Result<(), ServerError> {
        db.execute(
            "DELETE FROM apoio
            WHERE apoiador = $1 AND candidato = $2 AND ano = $3",
            &[&self.apoiador, &self.candidato, &self.ano],
        )
        .await?;
        Ok(())
    }

    /// Lista os apoios, com filtros opcionais
    pub async fn listar(
        db: &Client,
        filtro: ApoioFiltro,
        pagina: u16,
        limite: u16,
    ) -> Result<Vec<Apoio>, ServerError> {
        let filtro = filtro.cleanup();

        let query = format!(
            "SELECT apoiador, candidato, ano, funcao
            FROM apoio
            WHERE
                ($1::VARCHAR  IS NULL OR apoiador   = $1) AND
                ($2::VARCHAR  IS NULL OR candidato  = $2) AND
                ($3::SMALLINT IS NULL OR ano        = $3) AND
                ($4::VARCHAR  IS NULL OR funcao ILIKE $4)
            {} LIMIT $5 OFFSET $6",
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
                &filtro.apoiador,
                &filtro.candidato,
                &filtro.ano,
                &filtro.funcao,
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

/// Filtro de listagem de apoios
#[derive(Clone, Debug, Serialize, FromForm)]
pub struct ApoioFiltro {
    apoiador: Option<String>,
    candidato: Option<String>,
    ano: Option<i16>,
    funcao: Option<String>,
    ordenacao: Option<ApoioOrdenacao>,
    ordenacao_desc: bool,
}
impl ApoioFiltro {
    pub fn cleanup(self) -> Self {
        Self {
            apoiador: self.apoiador.filter(|s| !s.is_empty()),
            candidato: self.candidato.filter(|s| !s.is_empty()),
            funcao: self
                .funcao
                .filter(|s| !s.is_empty())
                .map(|s| format!("%{}%", s)),
            ..self
        }
    }
}

#[derive(Clone, Debug, Copy, Serialize, FromFormField, Display)]
#[strum(serialize_all = "snake_case")]
enum ApoioOrdenacao {
    Apoiador,
    Candidato,
    Ano,
    Funcao,
}
