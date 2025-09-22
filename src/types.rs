use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOptions {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub order_by: Option<String>,
    pub order_direction: Option<OrderDirection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub database_type: DatabaseType,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub pool_size: Option<u32>,
    pub timeout: Option<u64>,
    pub ssl: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseType {
    Postgres,
    MySQL,
    SQLite,
    MongoDB,
}

#[derive(Debug, Clone)]
pub struct QueryContext {
    pub user_id: Option<Uuid>,
    pub request_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

impl Default for QueryContext {
    fn default() -> Self {
        Self {
            user_id: None,
            request_id: Some(Uuid::new_v4()),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            limit: None,
            offset: None,
            order_by: None,
            order_direction: None,
        }
    }
}