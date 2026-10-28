use pointercrate_core::error::CoreError;
use serde::Serialize;
use sqlx::{postgres::types::PgInterval, prelude::FromRow, PgConnection};

#[derive(Debug, Serialize, FromRow)]
pub struct SubmissionAnalytics {
    list: String,
    submissions_30days: i64,
    approved_submissions_30days: i64,
}

impl SubmissionAnalytics {
    pub async fn log_submission(list: &str, record_id: &i32, connection: &mut PgConnection) -> Result<(), CoreError> {
        sqlx::query("INSERT INTO submission_analytics (list, record_id) VALUES ($1, $2)")
            .bind(list)
            .bind(record_id)
            .execute(&mut *connection)
            .await?;

        Ok(())
    }

    pub async fn get_submission_analytics(days: i32, connection: &mut PgConnection) -> Result<Vec<SubmissionAnalytics>, CoreError> {
        Ok(sqlx::query_as::<_, SubmissionAnalytics>(
            r#"
            SELECT
                a.list::text AS list,
                COUNT(*) AS submissions_30days,
                COUNT(*) FILTER (WHERE r.status_ = 'APPROVED') AS approved_submissions_30days
            FROM submission_analytics a
            LEFT JOIN records r ON r.id = a.record_id
            WHERE a.time >= (NOW() AT TIME ZONE 'utc') - INTERVAL $1
            GROUP BY a.list
            "#,
        )
        .bind(PgInterval {
            months: 0,
            days,
            microseconds: 0,
        })
        .fetch_all(&mut *connection)
        .await?)
    }
}
