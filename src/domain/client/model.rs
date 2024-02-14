use sea_query::Iden;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Client {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClientMeta {
    pub client_id: u32,
    pub negative_limit: i32,
    pub balance: i32,
}

#[derive(Copy, Clone, Iden, PartialEq)]
pub enum ClientTable {
    #[iden = "clients"]
    Table,
    ID,
    Name,
}

#[derive(Copy, Clone, Iden, PartialEq)]
pub enum ClientMetaTable {
    #[iden = "client_meta"]
    Table,
    ID,
    Balance,
    NegativeLimit,
}
