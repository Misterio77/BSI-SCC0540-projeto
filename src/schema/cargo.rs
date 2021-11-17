use postgres_types::{FromSql, ToSql};
use rust_decimal::Decimal;
use serde::Serialize;
use rocket::form::FromFormField;
use strum::EnumString;
use std::convert::{TryFrom, TryInto};

use crate::database::{Client, Row};
use crate::error::ServerError;

#[derive(Debug, Serialize)]
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
    pub async fn listar(db: &Client, filtro: CargoFiltro) -> Result<Vec<Cargo>, ServerError> {
        db.query(
            "
            SELECT tipo, local, cadeiras, salario
            FROM cargo
            WHERE
                ($1::tipo_cargo IS NULL OR tipo = $1) AND
                ($2::VARCHAR IS NULL OR local LIKE '%$2%') AND
                ($3::SMALLINT IS NULL OR cadeiras >= $3) AND
                ($4::SMALLINT IS NULL OR cadeiras <= $4) AND
                ($5::NUMERIC IS NULL OR salario >= $5) AND
                ($6::NUMERIC IS NULL OR salario <= $6)
            ",
            &[
                &filtro.tipo,
                &filtro.local,
                &filtro.min_cadeiras,
                &filtro.max_cadeiras,
                &filtro.min_salario,
                &filtro.max_salario,
            ],
        )
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }
    /// Cria um filtro para o metodo listar, pode ser manipulado usando os metodos dele
    pub fn filtro() -> CargoFiltro {
        CargoFiltro::default()
    }
}

/// Filtro de listagem de cargos
#[derive(Default, Serialize, Debug)]
pub struct CargoFiltro {
    pub tipo: Option<TipoCargo>,
    pub local: Option<String>,
    pub min_cadeiras: Option<i16>,
    pub max_cadeiras: Option<i16>,
    pub min_salario: Option<Decimal>,
    pub max_salario: Option<Decimal>,
}
