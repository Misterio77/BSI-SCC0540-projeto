use chrono::NaiveDate;
use serde::Serialize;
use std::convert::TryFrom;

use crate::error::{Result, ServerError};
use crate::database::{Client, Row};

use super::candidatura::Candidatura;

/// Indivíduo cadastrado no sistema
#[derive(Debug, Serialize)]
pub struct Individuo {
    /// CPF ou CNPJ
    id: String,
    nome: String,
    nascimento: NaiveDate,
    ficha_limpa: bool,
}

impl Individuo {
    /// Obter indivíduo
    pub async fn obter(db: &Client, id: &str) -> Result<Individuo> {
        db.query_one(
            "
            SELECT id, nome, nascimento, ficha_limpa
            FROM individuo
            WHERE id = $1",
            &[&id],
        )
        .await
        .map(Individuo::try_from)?
    }

    /// Lista os indivíduos, com filtros opcionais
    pub async fn listar(db: &Client) -> Result<Vec<Individuo>> {
        db.query(
            "
            SELECT id, nome, nascimento, ficha_limpa
            FROM individuo",
            &[],
        )
        .await?
        .into_iter()
        .map(Individuo::try_from)
        .collect()
    }

    // === Obter entidades relacionadas ===
    /// Retorna todas as candidaturas do individuo
    pub async fn candidaturas(&self, db: &Client) -> Result<Vec<Candidatura>> {
        Candidatura::listar(db, Some(&self.id), None, None, None, None, None, None).await
    }
    /*
    /// Retorna todas as vice candidaturas do individuo
    pub async fn vice_candidaturas(&self, db: &Client) -> Result<Vec<Candidatura>> {}
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
            id: row.try_get("id")?,
            nome: row.try_get("nome")?,
            nascimento: row.try_get("nascimento")?,
            ficha_limpa: row.try_get("ficha_limpa")?,
        })
    }
}
