use chrono::NaiveDate;
use serde::Serialize;
use std::convert::{TryFrom, TryInto};

use crate::database::{Client, Row};
use crate::error::ServerError;

/// Indivíduo cadastrado no sistema
#[derive(Serialize)]
pub struct Individuo {
    pub cpfcnpj: String,
    pub nome: String,
    pub nascimento: NaiveDate,
    pub ficha_limpa: bool,
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Individuo {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Individuo, ServerError> {
        Ok(Individuo {
            cpfcnpj: row.try_get("cpfcnpj")?,
            nome: row.try_get("nome")?,
            nascimento: row.try_get("nascimento")?,
            ficha_limpa: row.try_get("ficha_limpa")?,
        })
    }
}

impl Individuo {
    /// Obter indivíduo
    pub async fn obter(db: &Client, cpfcnpj: &str) -> Result<Individuo, ServerError> {
        db.query_one(
            "
            SELECT cpfcnpj, nome, nascimento, ficha_limpa
            FROM individuo
            WHERE cpfcnpj = $1",
            &[&cpfcnpj],
        )
        .await?
        .try_into()
    }

    /// Lista os indivíduos, com filtros opcionais
    pub async fn listar(
        db: &Client,
        filtro: IndividuoFiltro,
    ) -> Result<Vec<Individuo>, ServerError> {
        let filtro = filtro.cleanup();
        db.query(
            "
            SELECT cpfcnpj, nome, nascimento, ficha_limpa
            FROM individuo
            WHERE
                ($1::VARCHAR IS NULL OR cpfcnpj     = $1) AND
                ($2::VARCHAR IS NULL OR nome    ILIKE $2) AND
                ($3::DATE    IS NULL OR nascimento  = $3) AND
                ($4::BOOLEAN IS NULL OR ficha_limpa = $4)",
            &[
                &filtro.cpfcnpj,
                &filtro.nome,
                &filtro.nascimento,
                &filtro.ficha_limpa,
            ],
        )
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }
}

/// Filtro de listagem de indivíduo
#[derive(Serialize)]
pub struct IndividuoFiltro {
    pub cpfcnpj: Option<String>,
    pub nome: Option<String>,
    pub nascimento: Option<NaiveDate>,
    pub ficha_limpa: Option<bool>,
}
impl IndividuoFiltro {
    pub fn cleanup(self) -> Self {
        Self {
            cpfcnpj: self.cpfcnpj.filter(|s| !s.is_empty()),
            nome: self.nome.filter(|s| !s.is_empty()).map(|s| format!("%{}%", s)),
            ..self
        }
    }
}
