use postgres_types::{FromSql, ToSql};
use serde::Serialize;
use std::convert::TryFrom;

use crate::error::{Result, ServerError};
use crate::database::{Client, Row};

use super::candidatura::Candidatura;

#[derive(Debug, Serialize)]
pub struct Cargo {
    tipo: TipoCargo,
    local: String,
    cadeiras: i16,
}

#[derive(Debug, Serialize, ToSql, FromSql)]
#[postgres(name = "tipo_cargo")]
pub enum TipoCargo {
    Prefeito,
    Governador,
    Presidente,
    Vereador,
    #[serde(rename = "Deputado Estadual")]
    DeputadoEstadual,
    #[serde(rename = "Deputado Federal")]
    DeputadoFederal,
    Senador,
}

impl Cargo {
    /// Retorna um cargo, dado o tipo de cargo e local
    pub async fn obter(db: &Client, tipo: TipoCargo, local: &str) -> Result<Cargo> {
        db.query_one(
            "
            SELECT tipo, local, cadeiras
            FROM cargo
            WHERE tipo = $1 AND local = $2",
            &[&tipo, &local],
        )
        .await
        .map(Cargo::try_from)?
    }

    /// Lista os cargos
    pub async fn listar(db: &Client) -> Result<Vec<Cargo>> {
        db.query(
            "
            SELECT tipo, local, cadeiras
            FROM cargo",
            &[],
        )
        .await?
        .into_iter()
        .map(Cargo::try_from)
        .collect()
    }

    // === Obter entidades relacionadas ===
    /// Retorna as candidaturas pleiteando este cargo
    pub async fn candidaturas(&self, db: &Client) -> Result<Vec<Candidatura>> {
        db.query(
            "
            SELECT numero, ano, votos
            FROM candidatura
            WHERE cargo_tipo = $1 AND cargo_local = $2",
            &[&self.tipo, &self.local],
        )
        .await?
        .into_iter()
        .map(Candidatura::try_from)
        .collect()
    }
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Cargo {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Cargo> {
        Ok(Cargo {
            tipo: row.try_get("tipo")?,
            local: row.try_get("local")?,
            cadeiras: row.try_get("cadeiras")?,
        })
    }
}
