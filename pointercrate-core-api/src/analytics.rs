use pointercrate_core::{error::CoreError, pool::PointercratePool};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Cookie,
    Data, Request,
};
use serde::Serialize;
use sqlx::{prelude::FromRow, PgConnection};
use uuid::Uuid;

pub const DEVICE_COOKIE_NAME: &str = "pc-device-id";

#[derive(Debug, Serialize, FromRow)]
pub struct PageAnalytics {
    uri: String,
    visits_30days: i64,
    visitors_30days: i64, // visitors (identified by device id) in last 30 days
}

impl PageAnalytics {
    async fn log_visit(device_id: &str, uri: &str, connection: &mut PgConnection) -> Result<(), CoreError> {
        sqlx::query("INSERT INTO page_visits (device_id, uri) VALUES ($1::uuid, $2)")
            .bind(device_id)
            .bind(uri)
            .execute(&mut *connection)
            .await?;

        Ok(())
    }

    pub async fn get_page_visits(connection: &mut PgConnection) -> Result<Vec<PageAnalytics>, CoreError> {
        Ok(sqlx::query_as::<_, PageAnalytics>(
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

pub struct AnalyticsFairing(
    pub Vec<&'static str>, // the only pages to track visitor analytics for
);

#[rocket::async_trait]
impl Fairing for AnalyticsFairing {
    fn info(&self) -> Info {
        Info {
            name: "Analytics",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let uri = request.uri().to_string();
        if !self.0.contains(&uri.as_str()) {
            return;
        }

        let cookies = request.cookies();
        let device_id = match cookies.get(DEVICE_COOKIE_NAME) {
            Some(cookie) => cookie.value().to_string(),
            None => {
                let device_id = Uuid::new_v4().to_string();
                cookies.add(Cookie::build((DEVICE_COOKIE_NAME, device_id.clone())).permanent());

                device_id
            },
        };

        if let Some(pool) = request.rocket().state::<PointercratePool>() {
            if let Ok(mut connection) = pool.connection().await {
                PageAnalytics::log_visit(&device_id, &uri, &mut connection)
                    .await
                    .unwrap_or_else(|_| log::error!("Failed to log visit to {} for device {}", uri, &device_id));

                return;
            }
        }

        log::error!(
            "Failed to retrieve connection for analytics, visit to {} not logged (device id {})",
            uri,
            device_id
        );
    }
}
