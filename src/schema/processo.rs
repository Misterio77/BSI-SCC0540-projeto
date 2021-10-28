use crate::error::Result;

use super::individuo::Individuo;

use chrono::{Date, Utc};

/// Processo judicial
pub struct Processo {
    pub id: i32,
    pub crime: String,
    pub julgado: bool,
    pub data_julgamento: Option<Date<Utc>>,
    pub procedente: Option<bool>,
    pub pena: Option<i16>,
    pub multa: Option<i32>,
}

impl Processo {
    /// Obtém um processo, dado seu id
    pub fn obter(id: i32) -> Result<Processo> {}

    /// Lista os processos, com filtros opcionais
    pub fn listar(
        crime: Option<&str>,
        julgado: Option<bool>,
        procedente: Option<bool>,
    ) -> Result<Processo> {
    }

    // === Obter entidades relacionadas ===
    /// Retorna o réu do processo
    pub fn reu(&self) -> Result<Individuo> {}
}
