use pointercrate_core::pool::PointercratePool;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Cookie, SameSite},
    Data, Request,
};
use uuid::Uuid;

use crate::analytics::visits::PageVisitAnalytics;

pub mod submissions;
pub mod visits;

pub const DEVICE_COOKIE_NAME: &str = "pc-device-id";

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
        let user_agent = match request.headers().get_one("User-Agent") {
            Some(user_agent) => user_agent,
            None => return,
        };
        if !self.0.contains(&uri.as_str()) || !user_agent.starts_with("Mozilla/") {
            return;
        }

        let cookies = request.cookies();
        let device_id = match cookies.get(DEVICE_COOKIE_NAME) {
            Some(cookie) => cookie.value().to_string(),
            None => {
                let device_id = Uuid::new_v4().to_string();
                cookies.add(
                    Cookie::build((DEVICE_COOKIE_NAME, device_id.clone()))
                        .permanent()
                        .same_site(SameSite::Lax),
                );

                device_id
            },
        };

        if let Some(pool) = request.rocket().state::<PointercratePool>() {
            if let Ok(mut connection) = pool.connection().await {
                PageVisitAnalytics::log_visit(&device_id, &uri, Some(user_agent), &mut connection)
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
