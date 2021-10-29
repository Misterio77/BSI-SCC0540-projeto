use crate::error::Result;

use super::{candidatura::Candidatura, partido::Partido};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Cargo {
    pub tipo: TipoCargo,
    pub local: String,
    pub cadeiras: i16,
}

#[derive(Debug, Clone, strum::Display, Serialize)]
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
    /*
    /// Retorna um cargo, dado o tipo de poder e local
    pub fn obter(poder: Poder, local: &str) -> Result<Cargo> {}

    /// Lista os poderes, com filtro(s) opcional(is)
    pub fn listar(poder: Option<Poder>, local: Option<&str>) -> Result<Vec<Cargo>> {}

    // === Obter entidades relacionadas ===
    /// Retorna as candidaturas pleiteando este cargo, opcionalmente filtrando por ano e/ou partido
    pub fn candidaturas(
        &self,
        ano: Option<i32>,
        partido: Option<Partido>,
    ) -> Result<Vec<Candidatura>> {
    }
    */
}
