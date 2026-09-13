use crate::domain::{CollectionRule, Game};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};

pub struct LibraryStore {
    pool: SqlitePool,
}

impl LibraryStore {
    pub async fn in_memory() -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await?;
        sqlx::query("CREATE TABLE games (app_id INTEGER PRIMARY KEY, name TEXT NOT NULL, playtime_minutes INTEGER NOT NULL DEFAULT 0, installed INTEGER NOT NULL DEFAULT 0, tags_json TEXT NOT NULL DEFAULT '[]', approved_ip TEXT)")
            .execute(&pool).await?;
        sqlx::query("CREATE TABLE collections (name TEXT PRIMARY KEY, rule_json TEXT NOT NULL)")
            .execute(&pool)
            .await?;
        Ok(Self { pool })
    }

    pub async fn upsert_game(&self, app_id: u32, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO games (app_id, name) VALUES (?, ?) ON CONFLICT(app_id) DO UPDATE SET name = excluded.name")
            .bind(app_id).bind(name).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn approve_ip(&self, app_id: u32, ip: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE games SET approved_ip = ? WHERE app_id = ?")
            .bind(ip)
            .bind(app_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn approved_ip(&self, app_id: u32) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar("SELECT approved_ip FROM games WHERE app_id = ?")
            .bind(app_id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn save_game(&self, game: &Game) -> Result<(), sqlx::Error> {
        let tags = serde_json::to_string(&game.tags).expect("tags serialize");
        sqlx::query("INSERT INTO games (app_id,name,playtime_minutes,installed,tags_json,approved_ip) VALUES (?,?,?,?,?,?) ON CONFLICT(app_id) DO UPDATE SET name=excluded.name, playtime_minutes=excluded.playtime_minutes, installed=excluded.installed, tags_json=excluded.tags_json, approved_ip=COALESCE(games.approved_ip,excluded.approved_ip)")
            .bind(game.app_id).bind(&game.name).bind(game.playtime_minutes).bind(game.installed).bind(tags).bind(&game.approved_ip).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn list_games(&self) -> Result<Vec<Game>, sqlx::Error> {
        let rows = sqlx::query("SELECT app_id,name,playtime_minutes,installed,tags_json,approved_ip FROM games ORDER BY name").fetch_all(&self.pool).await?;
        Ok(rows
            .into_iter()
            .map(|row| Game {
                app_id: row.get::<i64, _>(0) as u32,
                name: row.get(1),
                playtime_minutes: row.get::<i64, _>(2) as u32,
                installed: row.get(3),
                tags: serde_json::from_str(&row.get::<String, _>(4)).unwrap_or_default(),
                approved_ip: row.get(5),
            })
            .collect())
    }

    pub async fn save_collection(
        &self,
        name: &str,
        rule: &CollectionRule,
    ) -> Result<(), sqlx::Error> {
        let rule = serde_json::to_string(rule).expect("rule serializes");
        sqlx::query("INSERT INTO collections(name,rule_json) VALUES(?,?) ON CONFLICT(name) DO UPDATE SET rule_json=excluded.rule_json").bind(name).bind(rule).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn list_collections(&self) -> Result<Vec<(String, CollectionRule)>, sqlx::Error> {
        let rows = sqlx::query("SELECT name,rule_json FROM collections ORDER BY name")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| {
                let name: String = row.get(0);
                let json: String = row.get(1);
                (
                    name,
                    serde_json::from_str(&json).expect("stored rule is valid"),
                )
            })
            .collect())
    }
}
