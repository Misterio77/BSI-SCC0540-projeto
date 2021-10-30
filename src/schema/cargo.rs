use postgres_types::{FromSql, ToSql};
use serde::Serialize;
use std::convert::TryFrom;

use crate::database::{Client, Row};
use crate::error::{Result, ServerError};

use super::Candidatura;

#[derive(Debug, Serialize)]
pub struct Cargo {
    tipo: TipoCargo,
    local: String,
    cadeiras: i16,
}

#[derive(Debug, Serialize, ToSql, FromSql, Clone, Copy)]
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
    pub async fn listar(
        db: &Client,
        tipo: Option<&TipoCargo>,
        local: Option<&str>,
        cadeiras: Option<i16>,
    ) -> Result<Vec<Cargo>> {
        db.query(
            "
            SELECT tipo, local, cadeiras
            FROM cargo
            WHERE
                ($1::tipo_cargo IS NULL OR tipo = $1) AND
                ($2::VARCHAR IS NULL OR local = $2) AND
                ($3::SMALLINT IS NULL OR cadeiras = $3)
            ",
            &[&tipo, &local, &cadeiras],
        )
        .await?
        .into_iter()
        .map(Cargo::try_from)
        .collect()
    }

    // === Obter entidades relacionadas ===
    /// Retorna as candidaturas pleiteando este cargo
    pub async fn candidaturas(&self, db: &Client) -> Result<Vec<Candidatura>> {
        Candidatura::listar(db, Candidatura::filtro().cargo_local(&self.local).cargo_tipo(self.tipo))
        .await
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
