use crate::error::{ServerError, Result};
use crate::{DatabaseClient, DatabaseRow};

use super::{individuo::Individuo, partido::Partido, cargo::Cargo};

use std::convert::TryFrom;
use serde::Serialize;

/// Candidatura política
#[derive(Debug, Clone, Serialize)]
pub struct Candidatura {
    pub numero: i32,
    pub ano: i16,
    /// Votos do pleito
    pub votos: Option<i32>,
}

impl Candidatura {
    /// Obtém uma candidatura, dado cargo, número, e ano
    pub async fn obter(db: &DatabaseClient, cargo: &Cargo, numero: i32, ano: i16) -> Result<Candidatura> {
        db.query_one("
            SELECT numero, ano, votos
            FROM candidatura
            WHERE numero = $1 AND ano = $2 AND cargo_tipo = $3 AND cargo_local = $4
        ", &[&numero, &ano, &cargo.tipo.to_string(), &cargo.local])
        .await.map(Candidatura::try_from)?
    }
    /*
    /// Obtém uma candidatura, dado candidato e ano
    pub fn obter_do_candidato(candidato: &Individuo, ano: i16) -> Result<Candidatura> {}
    /// Obtém uma candidatura, dado vice candidato e ano
    pub fn obter_do_vice_candidato(vice_candidato: &Individuo, ano: i16) -> Result<Candidatura> {}

    /// Lista as candidaturas, com filtro opcional
    pub fn listar(ano: Option<i16>) -> Result<Vec<Candidatura>> {}

    // === Obter entidades relacionadas ===
    /// Retorna o partido da candidatura
    pub fn partido(&self) -> Result<Partido> {}
    /// Retorna o candidato da candidatura
    pub fn candidato(&self) -> Result<Individuo> {}
    /// Retorna o vice candidato da candidatura, caso exista
    pub fn vice_candidato(&self) -> Result<Option<Individuo>> {}
    /// Retorna os membros da equipe da candidatura
    pub fn equipe(&self) -> Result<Vec<Individuo>> {}
    /// Retorna os doadores da candidatura
    pub fn doadores(&self) -> Result<Vec<Individuo>> {}
    */
}

impl TryFrom<DatabaseRow> for Candidatura {
    type Error = ServerError;
    fn try_from(row: DatabaseRow) -> Result<Candidatura> {
        let numero = row.try_get("numero")?;
        let ano = row.try_get("ano")?;
        let votos = row.try_get("votos")?;
        Ok(Candidatura {
            numero,
            ano,
            votos,
        })
    }
}
