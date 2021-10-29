use rust_decimal::Decimal;
use serde::Serialize;

/// Doação para candidatura
#[derive(Debug, Serialize)]
pub struct Doacao {
    /// Identificador da doação
    id: i32,
    /// Valor (decimal garante precisão esperada para dinheiro)
    /// (Equivale ao NUMERIC do postgres)
    valor: Decimal,
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
