use rocket::form::{FromForm, FromFormField};
use serde::Serialize;
use std::convert::{TryFrom, TryInto};
use strum::Display;
use time::Date;

use crate::database::{Client, Row};
use crate::error::ServerError;

/// Indivíduo cadastrado no sistema
#[derive(Clone, Serialize)]
pub struct Individuo {
    pub cpfcnpj: String,
    pub nome: String,
    pub nascimento: Date,
}

/// Converte da linha para o nosso tipo
impl TryFrom<Row> for Individuo {
    type Error = ServerError;
    fn try_from(row: Row) -> Result<Individuo, ServerError> {
        Ok(Individuo {
            cpfcnpj: row.try_get("cpfcnpj")?,
            nome: row.try_get("nome")?,
            nascimento: row.try_get("nascimento")?,
        })
    }
}

impl Individuo {
    /// Obter indivíduo
    pub async fn obter(db: &Client, cpfcnpj: &str) -> Result<Individuo, ServerError> {
        db.query_one(
            "
            SELECT cpfcnpj, nome, nascimento
            FROM individuo
            WHERE cpfcnpj = $1",
            &[&cpfcnpj],
        )
        .await?
        .try_into()
    }

    /// Deletar individuo
    pub async fn remover(self, db: &Client) -> Result<(), ServerError> {
        db.execute(
            "DELETE FROM individuo
            WHERE cpfcnpj = $1",
            &[&self.cpfcnpj],
        )
        .await?;
        Ok(())
    }

    /// Lista os indivíduos, com filtros opcionais
    pub async fn listar(
        db: &Client,
        filtro: IndividuoFiltro,
        pagina: u16,
        limite: u16,
    ) -> Result<Vec<Individuo>, ServerError> {
        let filtro = filtro.cleanup();

        let ficha_suja = "
            SELECT cpfcnpj, nome, nascimento
            FROM individuo
            INNER JOIN processo
                ON processo.reu = individuo.cpfcnpj
            INNER JOIN julgamento
                ON julgamento.processo = processo.id
            WHERE
                julgamento.procedente IS true AND
                julgamento.data >= (CURRENT_DATE - interval '5 years')";

        let query = format!(
            "SELECT cpfcnpj, nome, nascimento
            FROM individuo
            WHERE
                ($1::VARCHAR IS NULL OR cpfcnpj     = $1) AND
                ($2::VARCHAR IS NULL OR nome    ILIKE $2) AND
                ($3::DATE    IS NULL OR nascimento  = $3)
            {}
            {} LIMIT $4 OFFSET $5
            ",
            // Caso seja apenas ficha limpa, tirar os ficha suja
            if filtro.ficha_limpa {
                format!("EXCEPT ({})", ficha_suja)
            } else {
                "".to_string()
            },
            // Caso tenha ordenação, adicionar ORDER BY nome
            if let Some(ord) = filtro.ordenacao {
                format!(
                    "ORDER BY {} {}",
                    ord,
                    if filtro.ordenacao_desc { "DESC" } else { "" }
                )
            } else {
                "".to_string()
            },
        );

        db.query(
            &query,
            &[
                &filtro.cpfcnpj,
                &filtro.nome,
                &filtro.nascimento,
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

/// Filtro de listagem de indivíduo
#[derive(Clone, Serialize, FromForm)]
pub struct IndividuoFiltro {
    cpfcnpj: Option<String>,
    nome: Option<String>,
    nascimento: Option<Date>,
    ficha_limpa: bool,
    ordenacao: Option<IndividuoOrdenacao>,
    ordenacao_desc: bool,
}
impl IndividuoFiltro {
    pub fn cleanup(self) -> Self {
        Self {
            cpfcnpj: self.cpfcnpj.filter(|s| !s.is_empty()),
            nome: self
                .nome
                .filter(|s| !s.is_empty())
                .map(|s| format!("%{}%", s)),
            ..self
        }
    }
}

#[derive(Clone, Debug, Copy, Serialize, FromFormField, Display)]
#[strum(serialize_all = "snake_case")]
enum IndividuoOrdenacao {
    Cpfcnpj,
    Nome,
    Nascimento,
    Funcao,
}
