use crate::error::Result;

use super::{individuo::Individuo, partido::Partido};

/// Candidatura política
pub struct Candidatura {
    pub numero: i16,
    pub ano: i16,
    /// Votos do pleito
    pub votos: Option<i32>,
}

impl Candidatura {
    /// Obtém uma candidatura, dado número e ano
    pub fn obter(numero: i16, ano: i16) -> Result<Candidatura> {}
    /// Obtém uma candidatura, dado candidato e ano
    pub fn obter_do_candidato(candidato: &Individuo, ano: i16) -> Result<Candidatura> {}
    /// Obtém uma candidatura, dado vice candidato e ano
    pub fn obter_do_vice_candidato(vice_candidato: &Individuo, ano: i16) -> Result<Candidatura> {}

    /// Lista as candidaturas, com filtro opcional
    pub fn listar(ano: Option<i16>) -> Result<Vec<Candidatura>> {}

    // === Obter entidades relacionadas ===
    /// Retorna o partido da candidatura
    pub fn partido(&self) -> Result<Partido> {}
    /// Retorna o candidato da candidatura
    pub fn candidato(&self) -> Result<Individuo> {}
    /// Retorna o vice candidato da candidatura, caso exista
    pub fn vice_candidato(&self) -> Result<Option<Individuo>> {}
    /// Retorna os membros da equipe da candidatura
    pub fn equipe(&self) -> Result<Vec<Individuo>> {}
    /// Retorna os doadores da candidatura
    pub fn doadores(&self) -> Result<Vec<Individuo>> {}
}
