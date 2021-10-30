use serde::Serialize;
use std::convert::TryFrom;

use crate::database::{Client, Row};
use crate::error::{Result, ServerError};

use super::{Individuo, Partido, TipoCargo};

/// Candidatura política
#[derive(Debug, Serialize)]
pub struct Candidatura {
    candidato: String,
    vice_candidato: Option<String>,
    ano: i16,
    cargo_tipo: TipoCargo,
    cargo_local: String,
    numero: i32,
    partido: i16,
    votos: Option<i32>,
}

impl Candidatura {
    /// Obtém uma candidatura, dado candidato e ano
    pub async fn obter(db: &Client, candidato: &str, ano: i16) -> Result<Candidatura> {
        db.query_one(
            "SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido, votos
            FROM candidatura
            WHERE candidato = $1 AND ano = $2",
            &[&candidato, &ano],
        )
        .await
        .map(Candidatura::try_from)?
    }
    /// Obtém uma candidatura, dado vice candidato e ano
    pub async fn obter_do_vice(db: &Client, vice_candidato: &str, ano: i16) -> Result<Candidatura> {
        db.query_one(
            "SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido, votos
            FROM candidatura
            WHERE vice_candidato = $1 AND ano = $2",
            &[&vice_candidato, &ano],
        )
        .await
        .map(Candidatura::try_from)?
    }

    /// Obtém uma candidatura, dado cargo, número, e ano
    pub async fn obter_do_cargo(
        db: &Client,
        cargo_tipo: &TipoCargo,
        cargo_local: &str,
        numero: i32,
        ano: i16,
    ) -> Result<Candidatura> {
        db.query_one(
            "SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido, votos
            FROM candidatura
            WHERE cargo_tipo = $1 AND cargo_local = $2 AND numero = $3 AND ano = $4",
            &[&cargo_tipo, &cargo_local, &numero, &ano],
        )
        .await
        .map(Candidatura::try_from)?
    }

    /// Lista as candidaturas, com filtros opcionais
    pub async fn listar(
        db: &Client,
        candidato: Option<&str>,
        vice_candidato: Option<&str>,
        ano: Option<i16>,
        cargo_tipo: Option<&TipoCargo>,
        cargo_local: Option<&str>,
        numero: Option<i32>,
        partido: Option<i16>,
        votos: Option<i32>,
    ) -> Result<Vec<Candidatura>> {
        db.query(
            "
            SELECT candidato, vice_candidato, ano, cargo_tipo, cargo_local, numero, partido, votos
            FROM candidatura
            WHERE
                ($1::VARCHAR IS NULL OR candidato = $1) AND
                ($2::VARCHAR IS NULL OR vice_candidato = $2) AND
                ($3::SMALLINT IS NULL OR ano = $3) AND
                ($4::tipo_cargo IS NULL OR cargo_tipo = $4) AND
                ($5::VARCHAR IS NULL OR cargo_local = $5) AND
                ($6::INT IS NULL OR numero = $6) AND
                ($7::INT IS NULL OR votos = $7)
            ",
            &[
                &candidato,
                &vice_candidato,
                &ano,
                &cargo_tipo,
                &cargo_local,
                &numero,
                &partido,
                &votos,
            ],
        )
        .await?
        .into_iter()
        .map(Candidatura::try_from)
        .collect()
    }

    // === Obter entidades relacionadas ===
    /// Retorna o candidato da candidatura
    pub async fn candidato(&self, db: &Client) -> Result<Individuo> {
        Individuo::obter(db, &self.candidato).await
    }
    /// Retorna o vice candidato da candidatura, caso exista
    pub async fn vice_candidato(&self, db: &Client) -> Result<Option<Individuo>> {
        // Caso tenha um vice candidato
        if let Some(vc) = &self.vice_candidato {
            // Obter ele da base de dados
            Some(Individuo::obter(db, vc).await)
        } else {
            None
        }
        .transpose()
    }
    /// Retorna o partido da candidatura
    pub async fn partido(&self, db: &Client) -> Result<Partido> {
        Partido::obter(db, self.partido).await
    }
    /*
    /// Retorna os membros da equipe da candidatura
    pub fn equipe(&self) -> Result<Vec<Individuo>> {}
    /// Retorna os doadores da candidatura
    pub fn doadores(&self) -> Result<Vec<Individuo>> {}
    */
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Candidatura {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Candidatura> {
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
