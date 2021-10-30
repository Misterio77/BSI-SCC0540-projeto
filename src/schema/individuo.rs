use chrono::NaiveDate;
use serde::Serialize;
use std::convert::TryFrom;

use crate::database::{Client, Row};
use crate::error::{Result, ServerError};

use super::Candidatura;

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
    pub async fn listar(
        db: &Client,
        nome: Option<&str>,
        nascimento: Option<&NaiveDate>,
        ficha_limpa: Option<bool>,
    ) -> Result<Vec<Individuo>> {
        db.query(
            "
            SELECT id, nome, nascimento, ficha_limpa
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
        .map(Individuo::try_from)
        .collect()
    }

    // === Obter entidades relacionadas ===
    /// Retorna todas as candidaturas do individuo
    pub async fn candidaturas(&self, db: &Client) -> Result<Vec<Candidatura>> {
        Candidatura::listar(db,
            Some(&self.id),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ).await
    }
    /// Retorna todas as vice candidaturas do individuo
    pub async fn vice_candidaturas(&self, db: &Client) -> Result<Vec<Candidatura>> {
        Candidatura::listar(db,
            None,
            Some(&self.id),
            None,
            None,
            None,
            None,
            None,
            None,
        ).await
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
            id: row.try_get("id")?,
            nome: row.try_get("nome")?,
            nascimento: row.try_get("nascimento")?,
            ficha_limpa: row.try_get("ficha_limpa")?,
        })
    }
}
