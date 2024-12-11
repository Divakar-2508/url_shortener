use rusqlite::Connection;

use crate::models::UrlData;

pub struct ShortBase {
    conn: Connection,
}

impl ShortBase {
    pub fn new() -> Self {
        let conn = Connection::open("urls.db").unwrap();

        const QUERIES: &str = include_str!("../schema.sql");

        conn.execute_batch(QUERIES).unwrap();

        Self { conn }
    }

    pub fn insert_record(&self, url_data: &mut UrlData) -> Result<(), String> {
        let mut insert_query = self
            .conn
            .prepare(
                "INSERT INTO urls (url, short_code, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)
                RETURNING id",
            )
            .unwrap();

        let id = insert_query
            .query_row(
                [
                    &url_data.url,
                    &url_data.short_code,
                    &url_data.created_at,
                    &url_data.updated_at,
                ],
                |row| row.get::<&str, u32>("id"),
            )
            .map_err(|err| err.to_string())?;

        url_data.id = id;

        Ok(())
    }

    pub fn get_record(&self, short_code: &str) -> Option<UrlData> {
        let mut get_query = self
            .conn
            .prepare(
                "UPDATE urls SET access_count=access_count+1
                WHERE short_code = (?1)
                RETURNING id, url, created_at, updated_at, access_count",
            )
            .unwrap();

        let url_data = get_query.query_row([short_code], |row| {
            Ok(UrlData {
                id: row.get("id")?,
                url: row.get("url")?,
                short_code: short_code.to_string(),
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                access_count: Some(row.get("access_count")?),
            })
        });

        url_data.ok()
    }

    pub fn update_record(&self, short_code: &str, url: &str) -> Option<UrlData> {
        let mut update_query = self
            .conn
            .prepare(
                "UPDATE urls SET url = (?1)
                WHERE short_code = (?2)
                RETURNING id, created_at, updated_at",
            )
            .unwrap();

        let url_data = update_query.query_row([url, short_code], |row| {
            Ok(UrlData {
                id: row.get("id")?,
                url: url.to_string(),
                short_code: short_code.to_string(),
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                ..Default::default()
            })
        });

        url_data.ok()
    }

    pub fn delete_record(&self, short_code: &str) -> Option<()> {
        let mut delete_query = self
            .conn
            .prepare(
                "DELETE FROM urls
                WHERE short_code=(?1)",
            )
            .unwrap();

        delete_query.execute([short_code]).map(|_| ()).ok()
    }
}
