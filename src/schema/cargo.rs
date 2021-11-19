use postgres_types::{FromSql, ToSql};
use rocket::form::{FromForm, FromFormField};
use rust_decimal::Decimal;
use serde::Serialize;
use std::convert::{TryFrom, TryInto};
use strum::EnumString;

use crate::database::{Client, Row};
use crate::error::ServerError;

#[derive(Clone, Serialize)]
pub struct Cargo {
    pub tipo: TipoCargo,
    pub local: String,
    pub cadeiras: i16,
    pub salario: Decimal,
}

#[derive(Debug, Serialize, Clone, Copy, ToSql, FromSql, FromFormField, EnumString)]
#[postgres(name = "tipo_cargo")]
pub enum TipoCargo {
    Prefeito,
    Governador,
    Presidente,
    Vereador,
    #[serde(rename = "Deputado Estadual")]
    #[field(value = "Deputado Estadual")]
    #[strum(serialize = "Deputado Estadual")]
    DeputadoEstadual,
    #[serde(rename = "Deputado Federal")]
    #[field(value = "Deputado Federal")]
    #[strum(serialize = "Deputado Federal")]
    DeputadoFederal,
    Senador,
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Cargo {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Cargo, ServerError> {
        Ok(Cargo {
            tipo: row.try_get("tipo")?,
            local: row.try_get("local")?,
            cadeiras: row.try_get("cadeiras")?,
            salario: row.try_get("salario")?,
        })
    }
}

impl Cargo {
    /// Retorna um cargo, dado o tipo de cargo e local
    pub async fn obter(db: &Client, tipo: TipoCargo, local: &str) -> Result<Cargo, ServerError> {
        db.query_one(
            "
            SELECT tipo, local, cadeiras, salario
            FROM cargo
            WHERE tipo = $1 AND local = $2",
            &[&tipo, &local],
        )
        .await?
        .try_into()
    }

    /// Lista os cargos
    pub async fn listar(
        db: &Client,
        filtro: CargoFiltro,
        pagina: u16,
        limite: u16,
    ) -> Result<Vec<Cargo>, ServerError> {
        let filtro = filtro.cleanup();
        db.query(
            "
            SELECT tipo, local, cadeiras, salario
            FROM cargo
            WHERE
                ($1::tipo_cargo IS NULL OR tipo      = $1) AND
                ($2::VARCHAR    IS NULL OR local ILIKE $2) AND
                ($3::SMALLINT   IS NULL OR cadeiras >= $3) AND
                ($4::SMALLINT   IS NULL OR cadeiras <= $4) AND
                ($5::NUMERIC    IS NULL OR salario  >= $5) AND
                ($6::NUMERIC    IS NULL OR salario  <= $6)
            LIMIT $7 OFFSET $8",
            &[
                &filtro.tipo,
                &filtro.local,
                &filtro.min_cadeiras,
                &filtro.max_cadeiras,
                &filtro.min_salario,
                &filtro.max_salario,
                &(limite as i64),
                &(((pagina-1) as i64) * (limite as i64)),
            ],
        )
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }
}

/// Filtro de listagem de cargos
#[derive(Clone, Serialize, FromForm)]
pub struct CargoFiltro {
    pub tipo: Option<TipoCargo>,
    pub local: Option<String>,
    pub min_cadeiras: Option<i16>,
    pub max_cadeiras: Option<i16>,
    pub min_salario: Option<Decimal>,
    pub max_salario: Option<Decimal>,
}
impl CargoFiltro {
    pub fn cleanup(self) -> Self {
        Self {
            local: self
                .local
                .filter(|s| !s.is_empty())
                .map(|s| format!("%{}%", s)),
            ..self
        }
    }
}
