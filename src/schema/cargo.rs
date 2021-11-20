use rocket::{
    form::{FromForm, FromFormField},
    request::FromParam,
};
use rust_decimal::Decimal;
use serde::Serialize;
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

use postgres_types::{FromSql, ToSql};
use strum::{Display, EnumString};

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

impl FromParam<'_> for TipoCargo {
    type Error = ServerError;
    fn from_param(param: &str) -> Result<Self, Self::Error> {
        Ok(TipoCargo::from_str(param)?)
    }
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

    /// Deletar cargo
    pub async fn remover(self, db: &Client) -> Result<(), ServerError> {
        db.execute(
            "DELETE FROM cargo
            WHERE tipo = $1 AND local = $2",
            &[&self.tipo, &self.local],
        )
        .await?;
        Ok(())
    }

    /// Lista os cargos
    pub async fn listar(
        db: &Client,
        filtro: CargoFiltro,
        pagina: u16,
        limite: u16,
    ) -> Result<Vec<Cargo>, ServerError> {
        let filtro = filtro.cleanup();

        let query = format!(
            "SELECT tipo, local, cadeiras, salario
            FROM cargo
            WHERE
                ($1::tipo_cargo IS NULL OR tipo      = $1) AND
                ($2::VARCHAR    IS NULL OR local ILIKE $2) AND
                ($3::SMALLINT   IS NULL OR cadeiras >= $3) AND
                ($4::SMALLINT   IS NULL OR cadeiras <= $4) AND
                ($5::NUMERIC    IS NULL OR salario  >= $5) AND
                ($6::NUMERIC    IS NULL OR salario  <= $6)
            {} LIMIT $7 OFFSET $8",
            // Caso tenha ordenação, adicionar ORDER BY nome
            if let Some(ord) = filtro.ordenacao {
                format!(
                    "ORDER BY {} {}",
                    ord,
                    if filtro.ordenacao_desc { "DESC" } else { "" }
                )
            } else {
                "".to_string()
            }
        );

        db.query(
            &query,
            &[
                &filtro.tipo,
                &filtro.local,
                &filtro.min_cadeiras,
                &filtro.max_cadeiras,
                &filtro.min_salario,
                &filtro.max_salario,
                &(limite as i64),
                &(((pagina - 1) as i64) * (limite as i64)),
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
    tipo: Option<TipoCargo>,
    local: Option<String>,
    min_cadeiras: Option<i16>,
    max_cadeiras: Option<i16>,
    min_salario: Option<Decimal>,
    max_salario: Option<Decimal>,
    ordenacao: Option<CargoOrdenacao>,
    ordenacao_desc: bool,
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

#[derive(Clone, Debug, Copy, Serialize, FromFormField, Display)]
#[strum(serialize_all = "snake_case")]
enum CargoOrdenacao {
    Tipo,
    Local,
    Cadeiras,
    Salario,
}
