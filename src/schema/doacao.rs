use crate::error::Result;

use super::{candidatura::Candidatura, individuo::Individuo};

use rust_decimal::Decimal;

/// Doação para candidatura
pub struct Doacao {
    /// Identificador da doação
    pub id: i32,
    /// Valor (decimal garante precisão esperada para dinheiro)
    /// (Equivale ao NUMERIC do postgres)
    pub valor: Decimal,
}

impl Doacao {
    /*
    /// Obtém doação, dado id
    pub fn obter(id: i32) -> Result<Doacao> {}

    /// Lista as doações, com filtros opcionais
    pub fn listar(
        candidatura: Option<i16>,
        ano: Option<i16>,
        valor: Option<Decimal>,
    ) -> Result<Vec<Doacao>> {
    }

    // === Obter entidades relacionadas ===
    /// Retorna a candidatura cuja doação foi destinada
    pub fn candidatura(&self) -> Result<Candidatura> {}
    /// Retorna o indivíduo que fez a doação
    pub fn doador(&self) -> Result<Individuo> {}
    */
}
