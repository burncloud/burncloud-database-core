use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;

use crate::error::{DatabaseError, Result};

#[derive(Clone)]
pub struct DatabaseConnection {
    pool: SqlitePool,
}

impl DatabaseConnection {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub async fn close(self) {
        self.pool.close().await;
    }
}

pub struct Database {
    connection: Option<DatabaseConnection>,
    database_path: String,
}

impl Database {
    pub fn new<P: AsRef<Path>>(database_path: P) -> Self {
        let path = database_path.as_ref().to_string_lossy().to_string();
        Self {
            connection: None,
            database_path: path,
        }
    }

    pub fn new_in_memory() -> Self {
        Self {
            connection: None,
            database_path: ":memory:".to_string(),
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        let database_url = if self.database_path == ":memory:" {
            "sqlite::memory:".to_string()
        } else {
            format!("sqlite:{}", self.database_path)
        };

        let connection = DatabaseConnection::new(&database_url).await?;

        self.connection = Some(connection);
        Ok(())
    }

    pub fn connection(&self) -> Result<&DatabaseConnection> {
        self.connection
            .as_ref()
            .ok_or(DatabaseError::NotInitialized)
    }

    pub async fn create_tables(&self) -> Result<()> {
        let _conn = self.connection()?;

        Ok(())
    }

    pub async fn close(mut self) -> Result<()> {
        if let Some(connection) = self.connection.take() {
            connection.close().await;
        }
        Ok(())
    }

    pub async fn execute_query(&self, query: &str) -> Result<sqlx::sqlite::SqliteQueryResult> {
        let conn = self.connection()?;
        let result = sqlx::query(query).execute(conn.pool()).await?;
        Ok(result)
    }

    pub async fn fetch_one<T>(&self, query: &str) -> Result<T>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
    {
        let conn = self.connection()?;
        let result = sqlx::query_as::<_, T>(query).fetch_one(conn.pool()).await?;
        Ok(result)
    }

    pub async fn fetch_all<T>(&self, query: &str) -> Result<Vec<T>>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
    {
        let conn = self.connection()?;
        let results = sqlx::query_as::<_, T>(query).fetch_all(conn.pool()).await?;
        Ok(results)
    }

    pub async fn fetch_optional<T>(&self, query: &str) -> Result<Option<T>>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
    {
        let conn = self.connection()?;
        let result = sqlx::query_as::<_, T>(query).fetch_optional(conn.pool()).await?;
        Ok(result)
    }
}

pub async fn create_database<P: AsRef<Path>>(path: P) -> Result<Database> {
    let mut db = Database::new(path);
    db.initialize().await?;
    Ok(db)
}

pub async fn create_in_memory_database() -> Result<Database> {
    let mut db = Database::new_in_memory();
    db.initialize().await?;
    Ok(db)
}