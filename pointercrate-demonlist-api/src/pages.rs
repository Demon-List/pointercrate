use std::collections::HashMap;

use pointercrate_core_macros::{localized, themed};
use rocket::{response::Redirect, State};

<<<<<<< HEAD
use chrono::{DateTime, FixedOffset, NaiveDate, Utc};
use pointercrate_core::{audit::AuditLogEntryType, pool::PointercratePool};
=======
use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, Utc};
use pointercrate_core::{audit::AuditLogEntryType, pool::PointercratePool, theme::Theme};
>>>>>>> pr-304
use pointercrate_core_api::{
    error::Result,
    response::{Page, Response2},
};
use pointercrate_demonlist::player::{FullPlayer, Player};
use pointercrate_demonlist::{
    demon::{audit::audit_log_for_demon, current_list, list_at, FullDemon, MinimalDemon},
    error::DemonlistError,
    nationality::Nationality,
    LIST_ADMINISTRATOR, LIST_HELPER, LIST_MODERATOR,
};
use pointercrate_demonlist::{list::List, player::claim::PlayerClaim};
use pointercrate_demonlist_pages::{
    components::{team::Team, time_machine::Tardis},
    demon_page::{DemonMovement, DemonPage},
    overview::OverviewPage,
    statsviewer::individual::IndividualStatsViewer,
};
use pointercrate_integrate::gd::GeometryDashConnector;
use pointercrate_user::auth::NonMutating;
use pointercrate_user::User;
use pointercrate_user_api::auth::Auth;
use rocket::{futures::StreamExt, http::CookieJar};
use sqlx::PgConnection;

use crate::list::{self, ClientList};

#[localized]
#[themed]
#[rocket::get("/<list>/?<timemachine>&<submitter>")]
pub async fn overview(
    pool: &State<PointercratePool>, timemachine: Option<bool>, submitter: Option<bool>, list: ClientList, cookies: &CookieJar<'_>,
    auth: Option<Auth<NonMutating>>,
) -> Result<Page> {
    // A few months before pointercrate first went live - definitely the oldest data we have
    let beginning_of_time = NaiveDate::from_ymd_opt(2017, 1, 4).unwrap().and_hms_opt(0, 0, 0).unwrap();

    let mut connection = pool.connection().await?;

    let demonlist = current_list(&list.0, &mut connection).await?;

    let specified_when = cookies
        .get("when")
        .and_then(|cookie| DateTime::<FixedOffset>::parse_from_rfc3339(cookie.value()).ok());

    let specified_when = match specified_when {
        Some(when) if when.naive_utc() < beginning_of_time => Some(DateTime::from_naive_utc_and_offset(beginning_of_time, *when.offset())),
        Some(when) if when >= Utc::now() => None,
        Some(when) => Some(when),
        _ => None,
    };

    let mut tardis = Tardis::new(timemachine.unwrap_or(false));

    if let Some(destination) = specified_when {
<<<<<<< HEAD
        let demons_then = list_at(&mut connection, destination.naive_utc()).await?;
        tardis.activate(destination, demons_then, true)
=======
        let demons_then = list_at(&mut connection, &list.0, destination.naive_utc()).await?;
        tardis.activate(list.0.to_owned(), destination, demons_then, !is_april_1st)
>>>>>>> pr-304
    }

    Ok(Page::new(list::inject_list_context(
        &list.0,
        OverviewPage {
            team: Team {
                admins: User::by_permission(LIST_ADMINISTRATOR, &mut connection).await?,
                moderators: User::by_permission(LIST_MODERATOR, &mut connection).await?,
                helpers: User::by_permission(LIST_HELPER, &mut connection).await?,
            },
            list: list.0,
            demonlist,
            time_machine: tardis,
            submitter_initially_visible: submitter.unwrap_or(false),
            claimed_player: match auth {
                Some(auth) => claimed_full_player(auth.user.user(), &mut connection).await,
                None => None,
            },
        },
    )))
}

async fn claimed_full_player(user: &User, connection: &mut PgConnection) -> Option<FullPlayer> {
    let claim = PlayerClaim::by_user(user.id, connection).await.ok().flatten()?;
    let player = Player::by_id(claim.player.id, connection).await.ok()?;

    player.upgrade(connection).await.ok()
}

#[localized]
#[rocket::get("/<list>/permalink/<demon_id>/")]
pub async fn demon_permalink(demon_id: i32, pool: &State<PointercratePool>, list: ClientList) -> Result<Redirect> {
    let mut connection = pool.connection().await?;

    let demon = MinimalDemon::by_id(demon_id, &mut connection).await?;

    let position = demon.position(&list.0).ok_or(DemonlistError::DemonNotFound { demon_id })?;

    Ok(Redirect::to(rocket::uri!("/", demon_page(&list.0.as_str(), position))))
}

#[localized]
#[themed]
#[rocket::get("/<list>/<position>/", rank = 2)]
pub async fn demon_page(
    position: i16, pool: &State<PointercratePool>, gd: &State<GeometryDashConnector>, list: ClientList,
) -> Result<Page> {
    let mut connection = pool.connection().await?;

    let full_demon = FullDemon::by_position(position, &list.0, &mut connection).await?;

    let audit_log = audit_log_for_demon(full_demon.demon.base.id, &mut connection).await?;

    let mut addition_time = None;

    let mut modifications = audit_log
        .iter()
        .filter_map(|entry| match entry.r#type {
            AuditLogEntryType::Modification(ref modification) => match if list.0 == List::Demonlist {
                modification.rated_position
            } else {
                modification.position
            } {
                Some(old_position) if old_position > 0 => Some(DemonMovement {
                    from_position: old_position,
                    at: entry.time,
                }),
                _ => None,
            },
            AuditLogEntryType::Addition => {
                addition_time = Some(entry.time);

                None
            },
            _ => None,
        })
        .collect::<Vec<_>>();

    if let Some(addition) = addition_time {
        modifications.insert(
            0,
            DemonMovement {
                from_position: modifications
                    .first()
                    .map(|m| m.from_position)
                    .unwrap_or(full_demon.demon.base.position(&list.0).unwrap()), // cannot panic
                at: addition,
            },
        );
    }

    Ok(Page::new(list::inject_list_context(
        &list.0,
        DemonPage {
            team: Team {
                admins: User::by_permission(LIST_ADMINISTRATOR, &mut connection).await?,
                moderators: User::by_permission(LIST_MODERATOR, &mut connection).await?,
                helpers: User::by_permission(LIST_HELPER, &mut connection).await?,
            },
            demonlist: current_list(&list.0, &mut connection).await?,
            list: list.0,
            movements: modifications,
            integration: gd.load_level_for_demon(&full_demon.demon).await,
            data: full_demon,
        },
    )))
}

#[localized]
#[themed]
#[rocket::get("/<list>/statsviewer/", rank = 1)]
pub async fn stats_viewer(pool: &State<PointercratePool>, list: ClientList) -> Result<Page> {
    let mut connection = pool.connection().await?;

    Ok(Page::new(list::inject_list_context(
        &list.0,
        IndividualStatsViewer {
            nationalities_in_use: Nationality::used(&mut connection).await?,
            list: list.0,
        },
    )))
}

#[localized]
#[themed]
#[rocket::get("/<list>/statsviewer/nations/")]
pub async fn nation_stats_viewer(list: ClientList) -> Page {
    Page::new(list::inject_list_context(
        &list.0,
        pointercrate_demonlist_pages::statsviewer::national::nation_based_stats_viewer(&list.0),
    ))
}

#[rocket::get("/<list>/statsviewer/heatmap.css")]
pub async fn heatmap_css(list: ClientList, pool: &State<PointercratePool>) -> Result<Response2<String>> {
    let mut connection = pool.connection().await?;
    let mut css = String::new();

    let mut nation_scores = HashMap::new();
    let mut nations_stream = sqlx::query!(
        r#"SELECT iso_country_code, CASE WHEN $1 THEN score ELSE ratedplus_score END as score
        FROM nationalities 
        WHERE CASE WHEN $1 THEN score ELSE ratedplus_score END > 0.0"#,
        list.0 == List::Demonlist
    )
    .fetch(&mut *connection);

    while let Some(row) = nations_stream.next().await {
        let row = row.map_err(DemonlistError::from)?;

        nation_scores.insert(row.iso_country_code, row.score.unwrap());
    }

    let Some(&max_nation_score) = nation_scores.values().max_by(|a, b| a.total_cmp(b)) else {
        // Not a single nation has a score > 0. This means there are no approved records. So return no CSS
        return Ok(Response2::new(css).with_header("Content-Type", "text/css"));
    };

    for (nation, &score) in &nation_scores {
        css.push_str(&make_css_rule(&list.0, &nation.to_lowercase(), score, max_nation_score));
    }

    // un-borrow `connection`
    drop(nations_stream);

    let mut subdivisions_stream = sqlx::query!(
        r#"SELECT nation, iso_code, CASE WHEN $1 THEN score ELSE ratedplus_score END as score 
        FROM subdivisions 
        WHERE CASE WHEN $1 THEN score ELSE ratedplus_score END > 0.0"#,
        list.0 == List::Demonlist,
    )
    .fetch(&mut *connection);

    while let Some(row) = subdivisions_stream.next().await {
        let row = row.map_err(DemonlistError::from)?;

        css.push_str(&make_css_rule(
            &list.0,
            &format!("{}-{}", row.nation, row.iso_code),
            row.score.unwrap(),
            *nation_scores.get(&row.nation).unwrap_or(&f64::INFINITY),
        ))
    }

    Ok(Response2::new(css).with_header("Content-Type", "text/css"))
}

fn make_css_rule(list: &List, code: &str, score: f64, highest_score: f64) -> String {
    // Artificially adjust the highest score so that score/high_score is never 1. If it were 1, the resulting
    // color will be equal to the "hover"/"selected" color, which looks bad.
    let highest_score = highest_score * 1.5;

    let color = |theme: Theme, i: usize| -> f64 {
        let base_rgb = match theme {
            Theme::Light => [0xda, 0xdc, 0xe0],
            Theme::Dark => [0x25, 0x2d, 0x3c],
        };
        let target_rgb = match list {
            List::Demonlist => [0x08, 0x81, 0xc6],
            List::RatedPlus => [0xb3, 0x02, 0x05],
        };

        base_rgb[i] as f64 + (target_rgb[i] - base_rgb[i]) as f64 * (score / highest_score)
    };

    format!(
        r#"
            .heatmapped #{0}, .heatmapped #{0} > path {{ fill: rgb({1}, {2}, {3}); }}
            :root[data-theme="dark"] #{0}, :root[data-theme="dark"] #{0} > path {{ fill: rgb({4}, {5}, {6}); }}
        "#,
        code,
        color(Theme::Light, 0),
        color(Theme::Light, 1),
        color(Theme::Light, 2),
        color(Theme::Dark, 0),
        color(Theme::Dark, 1),
        color(Theme::Dark, 2),
    )
}
