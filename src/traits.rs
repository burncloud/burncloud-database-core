use crate::error::DatabaseResult;
use crate::{QueryContext, QueryOptions};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[async_trait]
pub trait DatabaseConnection: Send + Sync {
    async fn connect(&mut self) -> DatabaseResult<()>;
    async fn disconnect(&mut self) -> DatabaseResult<()>;
    async fn is_connected(&self) -> bool;
    async fn ping(&self) -> DatabaseResult<()>;
}

#[async_trait]
pub trait QueryExecutor: Send + Sync {
    async fn execute_query(
        &self,
        query: &str,
        params: &[&dyn QueryParam],
        context: &QueryContext,
    ) -> DatabaseResult<QueryResult>;

    async fn execute_query_with_options(
        &self,
        query: &str,
        params: &[&dyn QueryParam],
        options: &QueryOptions,
        context: &QueryContext,
    ) -> DatabaseResult<QueryResult>;
}

#[async_trait]
pub trait TransactionManager: Send + Sync {
    type Transaction: Transaction;

    async fn begin_transaction(&self, context: &QueryContext) -> DatabaseResult<Self::Transaction>;
}

#[async_trait]
pub trait Transaction: Send + Sync {
    async fn commit(self) -> DatabaseResult<()>;
    async fn rollback(self) -> DatabaseResult<()>;
    async fn execute_query(
        &self,
        query: &str,
        params: &[&dyn QueryParam],
    ) -> DatabaseResult<QueryResult>;
}

#[async_trait]
pub trait Repository<T>: Send + Sync
where
    T: Send + Sync,
{
    async fn find_by_id(&self, id: &str, context: &QueryContext) -> DatabaseResult<Option<T>>;
    async fn find_all(&self, options: &QueryOptions, context: &QueryContext) -> DatabaseResult<Vec<T>>;
    async fn create(&self, entity: &T, context: &QueryContext) -> DatabaseResult<String>;
    async fn update(&self, id: &str, entity: &T, context: &QueryContext) -> DatabaseResult<()>;
    async fn delete(&self, id: &str, context: &QueryContext) -> DatabaseResult<()>;
    async fn exists(&self, id: &str, context: &QueryContext) -> DatabaseResult<bool>;
}

#[async_trait]
pub trait MigrationManager: Send + Sync {
    async fn run_migrations(&self) -> DatabaseResult<()>;
    async fn rollback_migration(&self, version: &str) -> DatabaseResult<()>;
    async fn get_migration_status(&self) -> DatabaseResult<Vec<MigrationInfo>>;
}

pub trait QueryParam: Send + Sync {
    fn as_string(&self) -> String;
    fn as_i64(&self) -> Option<i64>;
    fn as_f64(&self) -> Option<f64>;
    fn as_bool(&self) -> Option<bool>;
    fn as_bytes(&self) -> Option<&[u8]>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub rows: Vec<HashMap<String, serde_json::Value>>,
    pub rows_affected: u64,
    pub last_insert_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationInfo {
    pub version: String,
    pub name: String,
    pub applied_at: chrono::DateTime<chrono::Utc>,
    pub checksum: String,
}