use postgres_types::{FromSql, ToSql};
use serde::Serialize;
use rust_decimal::Decimal;
use std::convert::{TryInto, TryFrom};

use crate::database::{Client, Row};
use crate::error::{Result, ServerError};

#[derive(Debug, Serialize)]
pub struct Cargo {
    pub tipo: TipoCargo,
    pub local: String,
    pub cadeiras: i16,
    pub salario: Decimal,
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

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Cargo {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Cargo> {
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
    pub async fn obter(db: &Client, tipo: TipoCargo, local: &str) -> Result<Cargo> {
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
    pub async fn listar(db: &Client, filtro: CargoFiltro) -> Result<Vec<Cargo>> {
        db.query(
            "
            SELECT tipo, local, cadeiras, salario
            FROM cargo
            WHERE
                ($1::tipo_cargo IS NULL OR tipo = $1) AND
                ($2::VARCHAR IS NULL OR local LIKE '%$2%') AND
                ($3::SMALLINT IS NULL OR cadeiras >= $3) AND
                ($4::SMALLINT IS NULL OR cadeiras <= $3) AND
                ($5::NUMERIC IS NULL OR salario >= $3) AND
                ($6::NUMERIC IS NULL OR salario <= $3)
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
/// Funciona como um builder
#[derive(Default)]
pub struct CargoFiltro {
    tipo: Option<TipoCargo>,
    local: Option<String>,
    min_cadeiras: Option<i16>,
    max_cadeiras: Option<i16>,
    min_salario: Option<Decimal>,
    max_salario: Option<Decimal>,
}

impl CargoFiltro {
    pub fn tipo(self, tipo: TipoCargo) -> Self {
        Self {
            tipo: Some(tipo),
            ..self
        }
    }
    pub fn local(self, local: &str) -> Self {
        Self {
            local: Some(local.into()),
            ..self
        }
    }
    pub fn min_cadeiras(self, min_cadeiras: i16) -> Self {
        Self {
            min_cadeiras: Some(min_cadeiras),
            ..self
        }
    }
    pub fn max_cadeiras(self, max_cadeiras: i16) -> Self {
        Self {
            max_cadeiras: Some(max_cadeiras),
            ..self
        }
    }
    pub fn min_salario(self, min_salario: &Decimal) -> Self {
        Self {
            min_salario: Some(min_salario.clone()),
            ..self
        }
    }
    pub fn max_salario(self, max_salario: &Decimal) -> Self {
        Self {
            max_salario: Some(max_salario.clone()),
            ..self
        }
    }
}
