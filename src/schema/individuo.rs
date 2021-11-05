use chrono::NaiveDate;
use serde::Serialize;
use std::convert::{TryInto, TryFrom};

use crate::database::{Client, Row};
use crate::error::{Result, ServerError};

use super::Candidatura;

/// Indivíduo cadastrado no sistema
#[derive(Debug, Serialize)]
pub struct Individuo {
    cpfcnpj: String,
    nome: String,
    nascimento: NaiveDate,
    ficha_limpa: bool,
}

impl Individuo {
    /// Obter indivíduo
    pub async fn obter(db: &Client, cpfcnpj: &str) -> Result<Individuo> {
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
        nome: Option<&str>,
        nascimento: Option<&NaiveDate>,
        ficha_limpa: Option<bool>,
    ) -> Result<Vec<Individuo>> {
        db.query(
            "
            SELECT cpfcnpj, nome, nascimento, ficha_limpa
            FROM individuo
            WHERE
                ($1::VARCHAR IS NULL OR nome = $1) AND
                ($2::DATE IS NULL OR nascimento = $2) AND
                ($3::BOOLEAN IS NULL OR ficha_limpa = $3)
            ",
            &[&nome, &nascimento, &ficha_limpa],
        )
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }

    // === Obter entidades relacionadas ===
    /// Retorna todas as candidaturas do individuo
    pub async fn candidaturas(&self, db: &Client) -> Result<Vec<Candidatura>> {
        Candidatura::listar(db, Candidatura::filtro().candidato(&self.cpfcnpj))
        .await
    }
    /// Retorna todas as vice candidaturas do individuo
    pub async fn vice_candidaturas(&self, db: &Client) -> Result<Vec<Candidatura>> {
        Candidatura::listar(db, Candidatura::filtro().vice_candidato(&self.cpfcnpj))
        .await
    }
    /*
    /// Retorna os processos do idividuo, opcionalmente filtrando por procedente
    pub fn processos(&self, procedente: Option<bool>) -> Result<Vec<Processo>> {}
    /// Retorna as candidaturas onde é membro da equipe, opcionalmente filtrando por ano
    pub fn candidaturas_equipe(&self, ano: Option<i16>) -> Result<Vec<Candidatura>> {}
    /// Retorna as candidaturas onde é doador, opcionalmente filtrando por ano
    pub fn candidaturas_doador(&self, ano: Option<i16>) -> Result<Vec<Candidatura>> {}
    */
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Individuo {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Individuo> {
        Ok(Individuo {
            cpfcnpj: row.try_get("cpfcnpj")?,
            nome: row.try_get("nome")?,
            nascimento: row.try_get("nascimento")?,
            ficha_limpa: row.try_get("ficha_limpa")?,
        })
    }
}
