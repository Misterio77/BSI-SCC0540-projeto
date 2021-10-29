use chrono::NaiveDate;
use serde::Serialize;

/// Processo judicial
#[derive(Debug, Serialize)]
pub struct Processo {
    id: i32,
    crime: String,
    julgado: bool,
    data_julgamento: Option<NaiveDate>,
    procedente: Option<bool>,
    pena: Option<i16>,
    multa: Option<i32>,
}

impl Processo {
    /*
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
    */
}
