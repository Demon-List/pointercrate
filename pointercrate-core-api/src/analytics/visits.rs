use pointercrate_core::error::CoreError;
use serde::Serialize;
use sqlx::{postgres::types::PgInterval, prelude::FromRow, PgConnection};

#[derive(Debug, Serialize, FromRow)]
pub struct PageVisitAnalytics {
    uri: String,

    visits: i64,
    visitors: i64,

    from_home: i64,
    from_within: i64,
    from_web: i64,
    from_social: i64,
    from_direct: i64,
}

impl PageVisitAnalytics {
    pub async fn log_visit(
        device_id: &str, used_home_button: bool, uri: &str, referer: Option<&str>, user_agent: Option<&str>, connection: &mut PgConnection,
    ) -> Result<(), CoreError> {
        sqlx::query("INSERT INTO page_visits (device_id, uri, referer, user_agent, used_home_button) VALUES ($1::uuid, $2, $3, $4, $5)")
            .bind(device_id)
            .bind(uri)
            // arbitrarily limit these to 500 chars so ppl cant send massive headers to get logged
            .bind(referer.and_then(|v| Some(v.chars().take(500).collect::<String>())))
            .bind(user_agent.and_then(|v| Some(v.chars().take(500).collect::<String>())))
            .bind(used_home_button)
            .execute(&mut *connection)
            .await?;

        Ok(())
    }

    pub async fn get_page_visits(days: i32, connection: &mut PgConnection) -> Result<Vec<PageVisitAnalytics>, CoreError> {
        Ok(sqlx::query_as::<_, PageVisitAnalytics>(
            r#"
            SELECT
                uri,
                COUNT(*) AS visits,
                COUNT(DISTINCT device_id) AS visitors,

                COUNT(*) FILTER
                    (WHERE referer = ANY(ARRAY['https://pointercrate.com/', 'https://www.pointercrate.com/']))
                    AS from_home,
                COUNT(*) FILTER
                    (WHERE NOT used_home_button AND referer <> ALL(ARRAY['https://pointercrate.com/', 'https://www.pointercrate.com/'])
                    AND referer ~ ANY(ARRAY['//pointercrate.com/', '//www.pointercrate.com/']))
                    AS from_within,
                COUNT(*) FILTER
                    (WHERE referer ~ ANY(ARRAY['google.', 'bing.com', 'yahoo.com', 'duckduckgo.com', 'yandex.', 'baidu.com', 'ecosia.org', 'search.brave.com', 'startpage.com']))
                    AS from_web,
                COUNT(*) FILTER
                    (WHERE referer ~ ANY(ARRAY['youtube.com', 'youtu.be', 'discord.com', 'discordapp.com', 'reddit.com', 'twitter.com', '//x.com/', '//t.co/', 'facebook.com', 'instagram.com', 'tiktok.com', 'twitch.tv', 'bsky.app']))
                    AS from_social,
                COUNT(*) FILTER (WHERE referer IS NULL) AS from_direct
            FROM page_visits
            WHERE time >= NOW() - $1::interval
            GROUP BY uri"#,
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
