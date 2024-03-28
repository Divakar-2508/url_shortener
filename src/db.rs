use sqlx::{
    Row,
    sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool
};

pub struct DBManager {
    pool: Pool<Sqlite>,
}

impl DBManager {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let db_url = "short_url.db";

        let create_query = "
            CREATE TABLE IF NOT EXISTS urls(
                short_url TEXT PRIMARY KEY NOT NULL,
                target_url TEXT NOT NULL,
                date_created TEXT
            )
        ";

        let connection_options = SqliteConnectOptions::new()
            .filename(db_url)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(connection_options).await?;

        sqlx::query(create_query).execute(&pool).await?;

        Ok(
            Self {
                pool
            }
        )
    }

    pub async fn add_entry(&self, short_url: &str, target_url: &str) -> Result<String, String> {
        const INSERT_QUERY: &str = "INSERT INTO urls (short_url, target_url, date_created) VALUES($1, $2, date())";

        let result = sqlx::query(INSERT_QUERY)
            .bind(short_url)
            .bind(target_url)
            .execute(&self.pool).await;

        if let Err(err) = result {
            if err.as_database_error().unwrap().is_unique_violation() {
                Err("The given Short Url is already in use, Can't create a Entry".to_string())
            } else {
                Err(format!("Error occured: {}", err))
            }
        } else {
            Ok("Url Added Successfully".to_string())
        }
    }

    pub async fn get_entry(&self, short_url: &str) -> Result<String, ()> {
        const FETCH_QUERY: &str = "SELECT * FROM urls WHERE short_url = $1";

        let result = sqlx::query(FETCH_QUERY)
            .bind(short_url)
            .fetch_one(&self.pool).await;

        match result {
            Ok(row) => Ok(row.get("target_url")),
            Err(_err) => Err(())
        }
    }

    pub async fn delete_entry(&self, short_url: &str) -> Result<String, String> {
        const DELETE_QUERY: &str = "DELETE FROM urls WHERE short_url = $1";

        let result = sqlx::query(DELETE_QUERY)
            .bind(short_url)
            .execute(&self.pool).await.unwrap();

        if result.rows_affected() == 0 {
            Err("No Entry with the given url found".to_string())
        } else {
            Ok("Entry Deleted Successfully".to_string())
        }
    } 
}

pub async fn delete_all(pool: &Pool<Sqlite>) {
    sqlx::query("DELETE FROM urls")
        .execute(pool).await.unwrap();
}