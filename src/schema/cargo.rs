use crate::error::Result;

use super::{candidatura::Candidatura, partido::Partido};

pub struct Cargo {
    poder: Poder,
    local: String,
    cadeiras: i16,
}

pub enum Poder {
    Executivo,
    Legislativo,
}

impl Cargo {
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
}
