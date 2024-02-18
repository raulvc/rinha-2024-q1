use sea_query::Iden;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Client {
    pub id: u32,
    pub negative_limit: i32,
    pub balance: i32,
}

#[derive(Copy, Clone, Iden, PartialEq)]
pub enum ClientTable {
    #[iden = "clients"]
    Table,
    ID,
    NegativeLimit,
    Balance,
}
