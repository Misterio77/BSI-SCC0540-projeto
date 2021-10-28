use crate::error::Result;

use super::candidatura::Candidatura;
use super::processo::Processo;

use chrono::{Date, Utc};

/// Indivíduo cadastrado no sistema
pub struct Individuo {
    /// CPF ou CNPJ
    pub id: String,
    pub nome: String,
    pub nascimento: Date<Utc>,
    pub ficha_limpa: bool,
}

impl Individuo {
    /// Obter indivíduo
    pub fn obter(id: &str) -> Result<Individuo> {}

    /// Lista os indivíduos, com filtros opcionais
    pub fn listar(
        nome: Option<&str>,
        nascimento: Option<&Date<Utc>>,
        ficha_limpa: Option<bool>,
    ) -> Result<Individuo> {
    }

    // === Obter entidades relacionadas ===
    /// Retorna os processos do idividuo, opcionalmente filtrando por procedente
    pub fn processos(&self, procedente: Option<bool>) -> Result<Vec<Processo>> {}
    /// Retorna as candidaturas do individuo, opcionalmente filtrando por ano
    pub fn candidaturas_candidato(&self, ano: Option<i16>) -> Result<Vec<Candidatura>> {}
    /// Retorna as candidaturas onde é membro da equipe, opcionalmente filtrando por ano
    pub fn candidaturas_equipe(&self, ano: Option<i16>) -> Result<Vec<Candidatura>> {}
    /// Retorna as candidaturas onde é doador, opcionalmente filtrando por ano
    pub fn candidaturas_doador(&self, ano: Option<i16>) -> Result<Vec<Candidatura>> {}
}
