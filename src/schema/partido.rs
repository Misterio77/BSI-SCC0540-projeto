use crate::error::Result;

use super::candidatura::Candidatura;

/// Partido político
pub struct Partido {
    pub numero: u8,
    pub nome: String,
    pub programa: String,
}

impl Partido {
    /*
    /// Obtém um partido, dado seu número
    pub fn obter(numero: u8) -> Result<Partido> {}
    /// Obtém um partido, dado seu nome
    pub fn obter_do_nome(nome: &str) -> Result<Partido> {}

    /// Lista os partidos
    pub fn listar() -> Result<Vec<Partido>> {}

    // === Obter entidades relacionadas ===
    /// Retorna as candidaturas filiadas ao partido
    pub fn candidaturas(&self) -> Result<Vec<Candidatura>> {}
    */
}
