use derive_new::new;
use serde::Serialize;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Clone, new)]
pub struct Statement {
    #[serde(rename = "saldo")]
    pub balance: StatementBalance,

    #[serde(rename = "ultimas_transacoes")]
    pub transactions: Vec<StatementTransaction>,
}

#[derive(Debug, Serialize, Clone, new)]
pub struct StatementBalance {
    #[serde(rename = "total")]
    pub balance: i32,

    #[serde(rename = "limite")]
    pub negative_limit: i32,

    #[serde(rename = "data_extrato", with = "time::serde::rfc3339")]
    pub requested_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Clone)]
pub struct StatementTransaction {
    #[serde(rename = "valor")]
    pub amount: i32,

    #[serde(rename = "tipo")]
    pub operation: String,

    #[serde(rename = "descricao")]
    pub description: String,

    #[serde(rename = "realizada_em", with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}
