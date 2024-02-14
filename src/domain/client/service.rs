use std::sync::Arc;

use anyhow::Context;
use axum::http::StatusCode;
use derive_new::new;
use libsql::{de, Connection};
use sea_query::{Expr, Query, SqliteQueryBuilder};

use crate::domain::client::model::{Client, ClientMeta, ClientMetaTable, ClientTable};
use crate::tools::error::{CustomError, DomainError};

#[derive(new)]
pub struct ClientService {
    db: Arc<Connection>,
}

impl ClientService {
    pub async fn find(&self, id: u32) -> Result<Client, CustomError> {
        let query = Self::find_query(id);

        let row = self
            .db
            .query(&query, None)
            .await
            .context("failed to query for rows")?
            .next()
            .context("failed to retrieve next row")?;

        if let Some(client_row) = row {
            let client = de::from_row::<Client>(&client_row).context("failed to parse row")?;
            return Ok(client);
        }

        Err(DomainError::new(
            format!("No matching client for id {}", id),
            StatusCode::NOT_FOUND.as_u16(),
        ))?
    }

    pub async fn find_meta(
        &self,
        id: u32,
        conn: Option<&Connection>,
    ) -> Result<ClientMeta, CustomError> {
        let db = conn.unwrap_or(&self.db);
        let query = Self::find_meta_query(id);

        let row = db
            .query(&query, None)
            .await
            .context("failed to query for rows")?
            .next()
            .context("failed to retrieve next row")?;

        if let Some(meta_row) = row {
            let meta = de::from_row::<ClientMeta>(&meta_row).context("failed to parse row")?;
            return Ok(meta);
        }

        Err(DomainError::new(
            format!("No matching client meta for client id {}", id),
            StatusCode::NOT_FOUND.as_u16(),
        ))?
    }

    fn find_query(client_id: u32) -> String {
        Query::select()
            .columns([ClientTable::ID, ClientTable::Name])
            .from(ClientTable::Table)
            .and_where(Expr::col(ClientTable::ID).eq(client_id))
            .to_string(SqliteQueryBuilder)
            .to_owned()
    }

    fn find_meta_query(client_id: u32) -> String {
        Query::select()
            .columns([
                ClientMetaTable::ID,
                ClientMetaTable::Balance,
                ClientMetaTable::NegativeLimit,
            ])
            .from(ClientMetaTable::Table)
            .and_where(Expr::col(ClientMetaTable::ID).eq(client_id))
            .to_string(SqliteQueryBuilder)
            .to_owned()
    }

    pub fn balance_update_query(client_id: u32, balance: i32) -> String {
        Query::update()
            .table(ClientMetaTable::Table)
            .values([(ClientMetaTable::Balance, balance.into())])
            .and_where(Expr::col(ClientMetaTable::ID).eq(client_id))
            .to_string(SqliteQueryBuilder)
            .to_owned()
    }
}
