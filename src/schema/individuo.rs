use chrono::NaiveDate;
use serde::Serialize;
use std::convert::{TryInto, TryFrom};

use crate::database::{Client, Row};
use crate::error::{Result, ServerError};

/// Indivíduo cadastrado no sistema
#[derive(Debug, Serialize)]
pub struct Individuo {
    pub cpfcnpj: String,
    pub nome: String,
    pub nascimento: NaiveDate,
    pub ficha_limpa: bool,
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Individuo {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Individuo> {
        Ok(Individuo {
            cpfcnpj: row.try_get("cpfcnpj")?,
            nome: row.try_get("nome")?,
            nascimento: row.try_get("nascimento")?,
            ficha_limpa: row.try_get("ficha_limpa")?,
        })
    }
}

impl Individuo {
    /// Obter indivíduo
    pub async fn obter(db: &Client, cpfcnpj: &str) -> Result<Individuo> {
        db.query_one(
            "
            SELECT cpfcnpj, nome, nascimento, ficha_limpa
            FROM individuo
            WHERE cpfcnpj = $1",
            &[&cpfcnpj],
        )
        .await?
        .try_into()
    }

    /// Lista os indivíduos, com filtros opcionais
    pub async fn listar(db: &Client, filtro: IndividuoFiltro) -> Result<Vec<Individuo>> {
        db.query(
            "
            SELECT cpfcnpj, nome, nascimento, ficha_limpa
            FROM individuo
            WHERE
                ($1::VARCHAR IS NULL OR cpfcnpj = $1) AND
                ($2::VARCHAR IS NULL OR nome LIKE '%$2%') AND
                ($3::DATE IS NULL OR nascimento = $3) AND
                ($4::BOOLEAN IS NULL OR ficha_limpa = $4)
            ",
            &[
                &filtro.cpfcnpj,
                &filtro.nome,
                &filtro.nascimento,
                &filtro.ficha_limpa
            ],
        )
        .await?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
    }

    /// Cria um filtro para o metodo listar, pode ser manipulado usando os metodos dele
    pub fn filtro() -> IndividuoFiltro {
        IndividuoFiltro::default()
    }
}

/// Filtro de listagem de indivíduo
/// Funciona como um builder
#[derive(Default)]
pub struct IndividuoFiltro {
    cpfcnpj: Option<String>,
    nome: Option<String>,
    nascimento: Option<NaiveDate>,
    ficha_limpa: Option<bool>,
}

impl IndividuoFiltro {
    pub fn cpfcnpj(self, cpfcnpj: &str) -> Self {
        Self {
            cpfcnpj: Some(cpfcnpj.into()),
            ..self
        }
    }
    pub fn nome(self, nome: &str) -> Self {
        Self {
            nome: Some(nome.into()),
            ..self
        }
    }
    pub fn nascimento(self, nascimento: &NaiveDate) -> Self {
        Self {
            nascimento: Some(nascimento.clone()),
            ..self
        }
    }
    pub fn ficha_limpa(self, ficha_limpa: bool) -> Self {
        Self {
            ficha_limpa: Some(ficha_limpa),
            ..self
        }
    }
}
