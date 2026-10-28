use pointercrate_core::error::CoreError;
use serde::Serialize;
use sqlx::{prelude::FromRow, PgConnection};

#[derive(Debug, Serialize, FromRow)]
pub struct PageVisitAnalytics {
    uri: String,
    visits_30days: i64,
    visitors_30days: i64, // visitors (identified by device id) in last 30 days
}

impl PageVisitAnalytics {
    pub async fn log_visit(
        device_id: &str, uri: &str, referer: Option<&str>, user_agent: Option<&str>, connection: &mut PgConnection,
    ) -> Result<(), CoreError> {
        sqlx::query("INSERT INTO page_visits (device_id, uri, referer, user_agent) VALUES ($1::uuid, $2, $3, $4)")
            .bind(device_id)
            .bind(uri)
            // arbitrarily limit these to 500 chars so ppl cant send massive headers to get logged
            .bind(referer.and_then(|v| Some(v.chars().take(500).collect::<String>())))
            .bind(user_agent.and_then(|v| Some(v.chars().take(500).collect::<String>())))
            .execute(&mut *connection)
            .await?;

        Ok(())
    }

    pub async fn get_page_visits(connection: &mut PgConnection) -> Result<Vec<PageVisitAnalytics>, CoreError> {
        Ok(sqlx::query_as::<_, PageVisitAnalytics>(
            r#"
            SELECT uri, COUNT(*) AS visits_30days, COUNT(DISTINCT device_id) AS visitors_30days
            FROM page_visits
            WHERE time >= NOW() - INTERVAL '30 days'
            GROUP BY uri"#,
        )
        .fetch_all(&mut *connection)
        .await?)
    }
}
