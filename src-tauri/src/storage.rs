use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub struct LibraryStore {
    pool: SqlitePool,
}

impl LibraryStore {
    pub async fn in_memory() -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new().max_connections(1).connect("sqlite::memory:").await?;
        sqlx::query("CREATE TABLE games (app_id INTEGER PRIMARY KEY, name TEXT NOT NULL, approved_ip TEXT)")
            .execute(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn upsert_game(&self, app_id: u32, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO games (app_id, name) VALUES (?, ?) ON CONFLICT(app_id) DO UPDATE SET name = excluded.name")
            .bind(app_id).bind(name).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn approve_ip(&self, app_id: u32, ip: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE games SET approved_ip = ? WHERE app_id = ?").bind(ip).bind(app_id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn approved_ip(&self, app_id: u32) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar("SELECT approved_ip FROM games WHERE app_id = ?").bind(app_id).fetch_optional(&self.pool).await
    }
}
