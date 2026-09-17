use pointercrate_core::etag::Taggable;
use pointercrate_demonlist::{
    demon::{recompute_rated_positions, Demon, FullDemon},
    player::{recompute_scores, DatabasePlayer, FullPlayer},
    record::RecordStatus,
    LIST_MODERATOR,
};
use pointercrate_test::demonlist::{add_demon, add_simple_record};
use pointercrate_test::TestClient;
use pointercrate_user::auth::{AuthenticatedUser, PasswordOrBrowser};
use rocket::http::Status;
use sqlx::{PgConnection, Pool, Postgres};

fn ranking_contains(ranking: &[serde_json::Value], name: &str) -> bool {
    ranking.iter().any(|player| player["name"].as_str() == Some(name))
}

fn link_param(links: &str, rel: &str, param: &str) -> Option<i64> {
    links
        .split(',')
        .find(|segment| segment.trim_end().ends_with(&format!("rel={}", rel)))
        .and_then(|segment| {
            let query = &segment[segment.find('?')? + 1..segment.find('>')?];
            query.split('&').find_map(|pair| {
                let (key, value) = pair.split_once('=')?;
                (key == param).then(|| value.parse().ok())?
            })
        })
}

async fn rated_positions_in_order(connection: &mut PgConnection) -> Vec<i16> {
    sqlx::query!("SELECT rated_position FROM demons WHERE rated_position IS NOT NULL ORDER BY position")
        .fetch_all(connection)
        .await
        .unwrap()
        .into_iter()
        .map(|row| row.rated_position.unwrap())
        .collect()
}

async fn set_demon_rated(clnt: &TestClient, helper: &AuthenticatedUser<PasswordOrBrowser>, demon_id: i32, rated: bool) {
    let full: FullDemon = clnt
        .get(format!("/api/v2/demons/{}/", demon_id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;

    clnt.patch(format!("/api/v2/demons/{}/", demon_id), &serde_json::json!({"rated": rated}))
        .authorize_as(helper)
        .header("If-Match", full.etag_string())
        .expect_status(Status::Ok)
        .execute()
        .await;
}

#[sqlx::test(migrations = "../migrations")]
async fn test_rated_positions_contiguous_after_operations(pool: Pool<Postgres>) {
    let (_clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let player = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    add_demon("A", 1, 50, player.id, player.id, true, &mut connection).await;
    let b = add_demon("B", 2, 50, player.id, player.id, true, &mut connection).await;
    let c = add_demon("C", 3, 50, player.id, player.id, false, &mut connection).await;
    add_demon("D", 4, 50, player.id, player.id, true, &mut connection).await;

    recompute_rated_positions(&mut connection).await.unwrap();
    assert_eq!(rated_positions_in_order(&mut connection).await, vec![1, 2, 3]);

    sqlx::query!("UPDATE demons SET rated = FALSE WHERE id = $1", b)
        .execute(&mut *connection)
        .await
        .unwrap();
    recompute_rated_positions(&mut connection).await.unwrap();
    assert_eq!(rated_positions_in_order(&mut connection).await, vec![1, 2]);

    sqlx::query!("UPDATE demons SET rated = TRUE WHERE id = $1", c)
        .execute(&mut *connection)
        .await
        .unwrap();
    recompute_rated_positions(&mut connection).await.unwrap();
    assert_eq!(rated_positions_in_order(&mut connection).await, vec![1, 2, 3]);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_rerating_demon_restores_demonlist_score_and_position(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let demon = clnt
        .add_demon(&helper, "Bloodbath", 1, 100, "stardust1971", "stardust1971", true)
        .await;
    let verifier_id = demon.demon.verifier.id;
    let demon_id = demon.demon.base.id;

    let baseline: FullPlayer = clnt
        .get(format!("/api/v1/players/{}/", verifier_id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;
    let original_score = baseline.player.rated_score;
    assert_ne!(original_score, 0.0f64);

    set_demon_rated(&clnt, &helper, demon_id, false).await;

    let unrated: FullPlayer = clnt
        .get(format!("/api/v1/players/{}/", verifier_id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;
    assert_eq!(unrated.player.rated_score, 0.0f64);

    set_demon_rated(&clnt, &helper, demon_id, true).await;

    let rerated: FullPlayer = clnt
        .get(format!("/api/v1/players/{}/", verifier_id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;
    assert_eq!(
        rerated.player.rated_score, original_score,
        "Re-rating demon failed to restore the verifier's demonlist score"
    );

    let refetched: FullDemon = clnt
        .get(format!("/api/v2/demons/{}/", demon_id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;
    assert_eq!(
        refetched.demon.base.rated_position,
        Some(1),
        "Re-rating demon failed to restore its rated position"
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn test_records_filtered_by_rated_position_excludes_unrated(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();
    let rated_holder = DatabasePlayer::by_name_or_create("rated_holder", &mut connection).await.unwrap();
    let unrated_holder = DatabasePlayer::by_name_or_create("unrated_holder", &mut connection).await.unwrap();

    let rated = add_demon("Bloodbath", 1, 50, verifier.id, verifier.id, true, &mut connection).await;
    let unrated = add_demon("Bloodlust", 2, 50, verifier.id, verifier.id, false, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    let r1 = add_simple_record(100, rated_holder.id, rated, RecordStatus::Approved, &mut connection).await;
    let r2 = add_simple_record(100, unrated_holder.id, unrated, RecordStatus::Approved, &mut connection).await;

    let by_rated_position: Vec<serde_json::Value> = clnt
        .get("/api/v1/records/?demon_rated_position=1")
        .expect_status(Status::Ok)
        .get_result()
        .await;
    assert_eq!(by_rated_position.len(), 1);
    assert_eq!(by_rated_position[0]["id"].as_i64(), Some(r1 as i64));

    let by_position: Vec<serde_json::Value> = clnt
        .get("/api/v1/records/?demon_position=2")
        .expect_status(Status::Ok)
        .get_result()
        .await;
    assert_eq!(by_position.len(), 1);
    assert_eq!(by_position[0]["id"].as_i64(), Some(r2 as i64));
}

#[sqlx::test(migrations = "../migrations")]
async fn test_heatmap_css_serves_per_list_and_rejects_invalid_list(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let demon = clnt
        .add_demon(&helper, "Bloodbath", 1, 100, "stardust1971", "stardust1971", true)
        .await;

    clnt.patch_player(demon.demon.verifier.id, &helper, serde_json::json!({"nationality": "GB"}))
        .await
        .execute()
        .await;

    let demonlist_css = clnt
        .get("/demonlist/statsviewer/heatmap.css")
        .expect_status(Status::Ok)
        .execute()
        .await
        .into_string()
        .await
        .unwrap();
    let ratedplus_css = clnt
        .get("/ratedplus/statsviewer/heatmap.css")
        .expect_status(Status::Ok)
        .execute()
        .await
        .into_string()
        .await
        .unwrap();

    assert!(
        demonlist_css.contains("#gb"),
        "Demonlist heatmap missing scored nation: {}",
        demonlist_css
    );
    assert!(
        ratedplus_css.contains("#gb"),
        "Rated+ heatmap missing scored nation: {}",
        ratedplus_css
    );

    clnt.get("/notalist/statsviewer/heatmap.css")
        .expect_status(Status::UnprocessableEntity)
        .execute()
        .await;
}

#[sqlx::test(migrations = "../migrations")]
async fn test_invalid_list_param_is_rejected(pool: Pool<Postgres>) {
    let (clnt, _connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    clnt.get("/api/v1/players/ranking/?list=notalist")
        .expect_status(Status::BadRequest)
        .execute()
        .await;
}

#[sqlx::test(migrations = "../migrations")]
async fn test_unrated_demon_gives_only_ratedplus_score(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let demon = clnt
        .add_demon(&helper, "Bloodbath", 1, 100, "stardust1971", "stardust1971", false)
        .await;

    let player: FullPlayer = clnt
        .get(format!("/api/v1/players/{}/", demon.demon.verifier.id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;

    assert_ne!(player.player.score, 0.0f64, "Unrated demon failed to give verifier a rated+ score");
    assert_eq!(player.player.rated_score, 0.0f64, "Unrated demon gave verifier a demonlist score");
}

#[sqlx::test(migrations = "../migrations")]
async fn test_rated_demon_gives_both_scores(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let demon = clnt
        .add_demon(&helper, "Bloodbath", 1, 100, "stardust1971", "stardust1971", true)
        .await;

    let player: FullPlayer = clnt
        .get(format!("/api/v1/players/{}/", demon.demon.verifier.id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;

    assert_ne!(player.player.score, 0.0f64, "Rated demon failed to give verifier a rated+ score");
    assert_ne!(
        player.player.rated_score, 0.0f64,
        "Rated demon failed to give verifier a demonlist score"
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn test_patch_demon_to_unrated_removes_demonlist_score(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let demon = clnt
        .add_demon(&helper, "Bloodbath", 1, 100, "stardust1971", "stardust1971", true)
        .await;
    let verifier_id = demon.demon.verifier.id;

    let player: FullPlayer = clnt
        .get(format!("/api/v1/players/{}/", verifier_id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;

    assert_ne!(player.player.rated_score, 0.0f64);

    let full: FullDemon = clnt
        .get(format!("/api/v2/demons/{}/", demon.demon.base.id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;

    let patched: FullDemon = clnt
        .patch(
            format!("/api/v2/demons/{}/", demon.demon.base.id),
            &serde_json::json!({"rated": false}),
        )
        .authorize_as(&helper)
        .header("If-Match", full.etag_string())
        .expect_status(Status::Ok)
        .get_success_result()
        .await;

    assert!(!patched.demon.rated);

    let refetched: FullDemon = clnt
        .get(format!("/api/v2/demons/{}/", demon.demon.base.id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;

    assert_eq!(refetched.demon.base.rated_position, None, "Unrated demon retained a rated position");

    let player: FullPlayer = clnt
        .get(format!("/api/v1/players/{}/", verifier_id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;

    assert_eq!(
        player.player.rated_score, 0.0f64,
        "Unrating demon failed to remove the verifier's demonlist score"
    );
    assert_ne!(
        player.player.score, 0.0f64,
        "Unrating demon wrongly removed the verifier's rated+ score"
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn test_rated_position_skips_unrated_demons(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let player = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    add_demon("Bloodbath", 1, 100, player.id, player.id, true, &mut connection).await;
    add_demon("Bloodlust", 2, 100, player.id, player.id, false, &mut connection).await;
    add_demon("Cataclysm", 3, 100, player.id, player.id, true, &mut connection).await;

    recompute_rated_positions(&mut connection).await.unwrap();

    let (demons, _) = clnt.get("/api/v2/demons/listed/").get_pagination_result::<Demon>().await;

    assert_eq!(demons.len(), 3);

    assert!(demons[0].rated);
    assert_eq!(demons[0].base.rated_position, Some(1));

    assert!(!demons[1].rated);
    assert_eq!(demons[1].base.rated_position, None);

    assert!(demons[2].rated);
    assert_eq!(demons[2].base.rated_position, Some(2));
}

#[sqlx::test(migrations = "../migrations")]
async fn test_player_ranking_differs_by_list(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let (rated_demon, unrated_demon) = setup_ranking_tests(&mut connection).await;

    add_simple_record(
        100,
        player_id("rated_holder", &mut connection).await,
        rated_demon,
        RecordStatus::Approved,
        &mut connection,
    )
    .await;
    add_simple_record(
        100,
        player_id("unrated_holder", &mut connection).await,
        unrated_demon,
        RecordStatus::Approved,
        &mut connection,
    )
    .await;

    recompute_scores(&mut connection).await.unwrap();

    let (demonlist_ranking, _) = clnt
        .get("/api/v1/players/ranking/?list=demonlist")
        .get_pagination_result::<serde_json::Value>()
        .await;
    let (ratedplus_ranking, _) = clnt
        .get("/api/v1/players/ranking/?list=ratedplus")
        .get_pagination_result::<serde_json::Value>()
        .await;

    assert!(ranking_contains(&ratedplus_ranking, "rated_holder"));
    assert!(ranking_contains(&demonlist_ranking, "rated_holder"));

    assert!(ranking_contains(&ratedplus_ranking, "unrated_holder"));
    assert!(!ranking_contains(&demonlist_ranking, "unrated_holder"));
}

#[sqlx::test(migrations = "../migrations")]
async fn test_progress_record_respects_each_lists_own_main_list(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();
    let holder = DatabasePlayer::by_name_or_create("stardust1972", &mut connection).await.unwrap();

    for position in 1..=10 {
        add_demon(
            format!("Unrated {}", position),
            position,
            50,
            verifier.id,
            verifier.id,
            false,
            &mut connection,
        )
        .await;
    }

    let mut target = 0;
    for position in 11..=76 {
        target = add_demon(
            format!("Rated {}", position),
            position,
            50,
            verifier.id,
            verifier.id,
            true,
            &mut connection,
        )
        .await;
    }

    recompute_rated_positions(&mut connection).await.unwrap();

    add_simple_record(90, holder.id, target, RecordStatus::Approved, &mut connection).await;

    recompute_scores(&mut connection).await.unwrap();

    let player: FullPlayer = clnt
        .get(format!("/api/v1/players/{}/", holder.id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;

    assert_ne!(
        player.player.rated_score, 0.0f64,
        "Progress record on a demonlist main-list demon failed to give demonlist score"
    );
    assert_eq!(
        player.player.score, 0.0f64,
        "Progress record on a rated+ extended-list demon wrongly gave rated+ score"
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn test_banned_player_excluded_from_both_rankings(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let mut verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();
    add_demon("Bloodbath", 1, 100, verifier.id, verifier.id, true, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();
    recompute_scores(&mut connection).await.unwrap();

    verifier.ban(&mut connection).await.unwrap();
    recompute_scores(&mut connection).await.unwrap();

    let (demonlist_ranking, _) = clnt
        .get("/api/v1/players/ranking/?list=demonlist")
        .get_pagination_result::<serde_json::Value>()
        .await;
    let (ratedplus_ranking, _) = clnt
        .get("/api/v1/players/ranking/?list=ratedplus")
        .get_pagination_result::<serde_json::Value>()
        .await;

    assert!(
        !ranking_contains(&demonlist_ranking, "stardust1971"),
        "Banned player appears in demonlist ranking"
    );
    assert!(
        !ranking_contains(&ratedplus_ranking, "stardust1971"),
        "Banned player appears in rated+ ranking"
    );
}

async fn setup_ranking_tests(connection: &mut PgConnection) -> (i32, i32) {
    let verifier = DatabasePlayer::by_name_or_create("stardust1971", connection).await.unwrap();

    let rated_demon = add_demon("Bloodbath", 1, 100, verifier.id, verifier.id, true, connection).await;
    let unrated_demon = add_demon("Bloodlust", 2, 100, verifier.id, verifier.id, false, connection).await;

    recompute_rated_positions(connection).await.unwrap();

    (rated_demon, unrated_demon)
}

async fn player_id(name: &str, connection: &mut PgConnection) -> i32 {
    DatabasePlayer::by_name_or_create(name, connection).await.unwrap().id
}

#[sqlx::test(migrations = "../migrations")]
async fn test_ratedplus_ranking_pagination_bounds_are_list_aware(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();
    let holder_a = DatabasePlayer::by_name_or_create("holder_a", &mut connection).await.unwrap();
    let holder_b = DatabasePlayer::by_name_or_create("holder_b", &mut connection).await.unwrap();

    let unrated = add_demon("UnratedTop", 1, 100, verifier.id, verifier.id, false, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    add_simple_record(100, holder_a.id, unrated, RecordStatus::Approved, &mut connection).await;
    add_simple_record(100, holder_b.id, unrated, RecordStatus::Approved, &mut connection).await;

    recompute_scores(&mut connection).await.unwrap();

    let (demonlist_ranking, _) = clnt
        .get("/api/v1/players/ranking/?list=demonlist")
        .get_pagination_result::<serde_json::Value>()
        .await;
    assert!(
        demonlist_ranking.is_empty(),
        "no rated demon has a scoring record, so the demonlist ranking must be empty"
    );

    let (ratedplus_ranking, links) = clnt
        .get("/api/v1/players/ranking/?list=ratedplus")
        .get_pagination_result::<serde_json::Value>()
        .await;

    assert!(
        ratedplus_ranking.len() >= 2,
        "expected at least the two record holders in the rated+ ranking, got {:?}",
        ratedplus_ranking
    );

    let last_before = link_param(&links, "last", "before")
        .unwrap_or_else(|| panic!("rated+ ranking response had no usable 'last' link: {}", links));

    assert!(
        last_before > ratedplus_ranking.len() as i64,
        "'last' link points before index {}, but the rated+ ranking has {} players (Links: {})",
        last_before,
        ratedplus_ranking.len(),
        links
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn test_time_machine_filters_demons_per_list(pool: Pool<Postgres>) {
    use pointercrate_demonlist::demon::list_at;
    use pointercrate_demonlist::list::List;

    let (_clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let player = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    add_demon("RatedTop", 1, 100, player.id, player.id, true, &mut connection).await;
    add_demon("UnratedMid", 2, 100, player.id, player.id, false, &mut connection).await;
    add_demon("RatedLow", 3, 100, player.id, player.id, true, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    let when = sqlx::types::chrono::NaiveDate::from_ymd_opt(2100, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let demonlist = list_at(&mut connection, &List::Demonlist, when).await.unwrap();
    let mut demonlist_names: Vec<String> = demonlist.iter().map(|d| d.current_demon.base.name.clone()).collect();
    demonlist_names.sort();
    assert_eq!(
        demonlist_names,
        vec!["RatedLow".to_string(), "RatedTop".to_string()],
        "the demonlist time machine must contain only rated demons"
    );

    let mut demonlist_positions: Vec<i16> = demonlist.iter().map(|d| d.current_demon.base.position).collect();
    demonlist_positions.sort();
    assert_eq!(
        demonlist_positions,
        vec![1, 2],
        "the demonlist time machine must number rated demons by contiguous rated position"
    );

    let ratedplus = list_at(&mut connection, &List::RatedPlus, when).await.unwrap();
    let mut ratedplus_names: Vec<String> = ratedplus.iter().map(|d| d.current_demon.base.name.clone()).collect();
    ratedplus_names.sort();
    assert_eq!(
        ratedplus_names,
        vec!["RatedLow".to_string(), "RatedTop".to_string(), "UnratedMid".to_string()],
        "the rated+ time machine must contain every demon"
    );

    let mut ratedplus_positions: Vec<i16> = ratedplus.iter().map(|d| d.current_demon.base.position).collect();
    ratedplus_positions.sort();
    assert_eq!(
        ratedplus_positions,
        vec![1, 2, 3],
        "the rated+ time machine must number demons by overall position"
    );
}

fn ranking_has_country(ranking: &[serde_json::Value], iso: &str) -> bool {
    ranking.iter().any(|entry| entry["country_code"].as_str() == Some(iso))
}

#[sqlx::test(migrations = "../migrations")]
async fn test_ranking_pagination_last_bound_is_reachable_for_both_lists(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();
    let rated_holder = DatabasePlayer::by_name_or_create("rated_holder", &mut connection).await.unwrap();
    let unrated_holder = DatabasePlayer::by_name_or_create("unrated_holder", &mut connection).await.unwrap();

    let rated = add_demon("Rated", 1, 100, verifier.id, verifier.id, true, &mut connection).await;
    let unrated = add_demon("Unrated", 2, 100, verifier.id, verifier.id, false, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    add_simple_record(100, rated_holder.id, rated, RecordStatus::Approved, &mut connection).await;
    add_simple_record(100, unrated_holder.id, unrated, RecordStatus::Approved, &mut connection).await;
    recompute_scores(&mut connection).await.unwrap();

    let (demonlist_ranking, demonlist_links) = clnt
        .get("/api/v1/players/ranking/?list=demonlist")
        .get_pagination_result::<serde_json::Value>()
        .await;
    let (ratedplus_ranking, ratedplus_links) = clnt
        .get("/api/v1/players/ranking/?list=ratedplus")
        .get_pagination_result::<serde_json::Value>()
        .await;

    assert!(
        ratedplus_ranking.len() > demonlist_ranking.len(),
        "expected the rated+ ranking to have strictly more players than the demonlist ranking"
    );

    assert!(
        link_param(&demonlist_links, "last", "before").unwrap() > demonlist_ranking.len() as i64,
        "demonlist 'last' bound must be reachable past the last player (Links: {})",
        demonlist_links
    );
    assert!(
        link_param(&ratedplus_links, "last", "before").unwrap() > ratedplus_ranking.len() as i64,
        "rated+ 'last' bound must be reachable past the last player (Links: {})",
        ratedplus_links
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn test_nation_ranking_differs_by_list(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let demon = clnt
        .add_demon(&helper, "Bloodbath", 1, 100, "stardust1971", "stardust1971", false)
        .await;

    clnt.patch_player(demon.demon.verifier.id, &helper, serde_json::json!({"nationality": "GB"}))
        .await
        .execute()
        .await;

    let ratedplus_nations: Vec<serde_json::Value> = clnt
        .get("/api/v1/nationalities/ranking/?list=ratedplus")
        .expect_status(Status::Ok)
        .get_result()
        .await;
    let demonlist_nations: Vec<serde_json::Value> = clnt
        .get("/api/v1/nationalities/ranking/?list=demonlist")
        .expect_status(Status::Ok)
        .get_result()
        .await;

    assert!(
        ranking_has_country(&ratedplus_nations, "GB"),
        "an unrated demon's verifier nation must appear in the rated+ nation ranking"
    );
    assert!(
        !ranking_has_country(&demonlist_nations, "GB"),
        "an unrated demon's verifier nation must not appear in the demonlist nation ranking"
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn test_full_and_incremental_score_paths_agree(pool: Pool<Postgres>) {
    let (_clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();
    let holder = DatabasePlayer::by_name_or_create("holder", &mut connection).await.unwrap();

    let r1 = add_demon("R1", 1, 40, verifier.id, verifier.id, true, &mut connection).await;
    add_demon("U1", 2, 40, verifier.id, verifier.id, false, &mut connection).await;
    let r2 = add_demon("R2", 3, 40, verifier.id, verifier.id, true, &mut connection).await;
    let u2 = add_demon("U2", 4, 40, verifier.id, verifier.id, false, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    add_simple_record(100, holder.id, r1, RecordStatus::Approved, &mut connection).await;
    add_simple_record(70, holder.id, r2, RecordStatus::Approved, &mut connection).await;
    add_simple_record(100, holder.id, u2, RecordStatus::Approved, &mut connection).await;

    recompute_scores(&mut connection).await.unwrap();
    let full = sqlx::query!("SELECT score, ratedplus_score FROM players WHERE id = $1", holder.id)
        .fetch_one(&mut *connection)
        .await
        .unwrap();

    holder.update_score(&mut connection).await.unwrap();
    let incremental = sqlx::query!("SELECT score, ratedplus_score FROM players WHERE id = $1", holder.id)
        .fetch_one(&mut *connection)
        .await
        .unwrap();

    assert!(
        (full.score - incremental.score).abs() < 1e-9,
        "demonlist score drifts between recompute_scores ({}) and update_score ({})",
        full.score,
        incremental.score
    );
    assert!(
        (full.ratedplus_score - incremental.ratedplus_score).abs() < 1e-9,
        "rated+ score drifts between recompute_scores ({}) and update_score ({})",
        full.ratedplus_score,
        incremental.ratedplus_score
    );
    assert!(full.score > 0.0 && full.ratedplus_score > 0.0);
}

#[test]
fn test_demon_score_is_list_aware() {
    use pointercrate_demonlist::demon::{Demon, MinimalDemon};
    use pointercrate_demonlist::list::List;

    let player = DatabasePlayer {
        id: 1,
        name: "p".to_string(),
        banned: false,
    };

    let rated = Demon {
        base: MinimalDemon {
            id: 1,
            position: 200,
            rated_position: Some(5),
            name: "Rated".to_string(),
        },
        requirement: 50,
        video: None,
        thumbnail: String::new(),
        publisher: player.clone(),
        verifier: player.clone(),
        level_id: None,
        rated: true,
    };

    assert_eq!(
        rated.score(&List::RatedPlus, 100),
        0.0,
        "a demon past the rated+ extended list must be worth nothing on rated+"
    );
    assert!(
        rated.score(&List::Demonlist, 100) > 0.0,
        "a demon high on the demonlist must be worth points there"
    );

    let unrated = Demon {
        base: MinimalDemon {
            id: 2,
            position: 5,
            rated_position: None,
            name: "Unrated".to_string(),
        },
        rated: false,
        ..rated
    };

    assert_eq!(
        unrated.score(&List::Demonlist, 100),
        0.0,
        "an unrated demon must be worth nothing on the demonlist"
    );
    assert!(
        unrated.score(&List::RatedPlus, 100) > 0.0,
        "an unrated demon high on the rated+ list must be worth points there"
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn test_moving_demon_reorders_rated_positions(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let player = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    add_demon("A", 1, 100, player.id, player.id, true, &mut connection).await;
    add_demon("B", 2, 100, player.id, player.id, true, &mut connection).await;
    let c = add_demon("C", 3, 100, player.id, player.id, true, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    let full: FullDemon = clnt
        .get(format!("/api/v2/demons/{}/", c))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;

    clnt.patch(format!("/api/v2/demons/{}/", c), &serde_json::json!({"position": 1}))
        .authorize_as(&helper)
        .header("If-Match", full.etag_string())
        .expect_status(Status::Ok)
        .execute()
        .await;

    let (demons, _) = clnt.get("/api/v2/demons/listed/").get_pagination_result::<Demon>().await;

    let names: Vec<&str> = demons.iter().map(|d| d.base.name.as_str()).collect();
    assert_eq!(names, vec!["C", "A", "B"], "moving C to the top must reorder the list");

    let rated_positions: Vec<Option<i16>> = demons.iter().map(|d| d.base.rated_position).collect();
    assert_eq!(
        rated_positions,
        vec![Some(1), Some(2), Some(3)],
        "rated positions must stay contiguous and follow the new order after a move"
    );
}

async fn submit_as(
    clnt: &TestClient, helper: &AuthenticatedUser<PasswordOrBrowser>, demon: i32, progress: i16, expected: Status,
) -> serde_json::Value {
    let response = clnt
        .post(
            "/api/v1/records/",
            &serde_json::json!({
                "progress": progress,
                "player": "stardust1972",
                "demon": demon,
                "raw_footage": "https://example.com/raw"
            }),
        )
        .authorize_as(helper)
        .expect_status(expected)
        .execute()
        .await;

    serde_json::from_str(&response.into_string().await.unwrap()).unwrap_or(serde_json::Value::Null)
}

#[sqlx::test(migrations = "../migrations")]
async fn test_submit_non100_rejected_on_demon_legacy_on_both_lists(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();
    let legacy = add_demon("Legacy", 200, 10, verifier.id, verifier.id, false, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    let body = submit_as(&clnt, &helper, legacy, 50, Status::UnprocessableEntity).await;
    assert_eq!(body["code"].as_i64(), Some(42219), "expected SubmitLegacy, got {}", body);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_submit_non100_rejected_on_extended_only_demon(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();
    let extended = add_demon("Extended", 100, 10, verifier.id, verifier.id, false, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    let body = submit_as(&clnt, &helper, extended, 50, Status::UnprocessableEntity).await;
    assert_eq!(body["code"].as_i64(), Some(42220), "expected Non100Extended, got {}", body);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_submit_non100_accepted_on_top_unrated_demon(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();
    let top = add_demon("TopUnrated", 50, 10, verifier.id, verifier.id, false, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    let body = submit_as(&clnt, &helper, top, 50, Status::Ok).await;
    assert_eq!(body["data"]["progress"].as_i64(), Some(50), "expected the record to be created, got {}", body);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_time_machine_rated_position_survives_combined_move_and_unrate(pool: Pool<Postgres>) {
    use pointercrate_demonlist::demon::list_at;
    use pointercrate_demonlist::list::List;

    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;
    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let player = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    let d1 = add_demon("D1", 1, 100, player.id, player.id, true, &mut connection).await;
    add_demon("D2", 2, 100, player.id, player.id, true, &mut connection).await;
    let d3 = add_demon("D3", 3, 100, player.id, player.id, true, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    let before = sqlx::query!(r#"SELECT (NOW() AT TIME ZONE 'utc') AS "now!""#)
        .fetch_one(&mut *connection)
        .await
        .unwrap()
        .now;

    let full: FullDemon = clnt
        .get(format!("/api/v2/demons/{}/", d3))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;

    clnt.patch(format!("/api/v2/demons/{}/", d3), &serde_json::json!({"position": 1, "rated": false}))
        .authorize_as(&helper)
        .header("If-Match", full.etag_string())
        .expect_status(Status::Ok)
        .execute()
        .await;

    let mut reconstructed: Vec<(String, i16)> = list_at(&mut connection, &List::Demonlist, before)
        .await
        .unwrap()
        .into_iter()
        .map(|d| (d.current_demon.base.name, d.current_demon.base.position))
        .collect();
    reconstructed.sort_by_key(|(_, position)| *position);

    assert_eq!(
        reconstructed,
        vec![("D1".to_string(), 1), ("D2".to_string(), 2), ("D3".to_string(), 3)],
        "the rated list as of before the patch must reconstruct exactly, not an intermediate recompute value"
    );

    let intermediate = sqlx::query!("SELECT COUNT(*) AS c FROM demon_modifications WHERE id = $1 AND rated_position = 2", d1)
        .fetch_one(&mut *connection)
        .await
        .unwrap()
        .c
        .unwrap_or(0);
    assert!(
        intermediate >= 1,
        "expected the transient rated_position=2 for D1 to exist in the audit log, so the tiebreaker is actually exercised"
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn test_raising_requirement_updates_player_scores(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;
    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;

    let verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();
    let holder = DatabasePlayer::by_name_or_create("holder", &mut connection).await.unwrap();

    let demon_id = add_demon("Bloodbath", 1, 40, verifier.id, verifier.id, true, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();
    add_simple_record(50, holder.id, demon_id, RecordStatus::Approved, &mut connection).await;
    recompute_scores(&mut connection).await.unwrap();

    let before: FullPlayer = clnt
        .get(format!("/api/v1/players/{}/", holder.id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;
    assert_ne!(before.player.rated_score, 0.0f64, "sanity: the 50% record should give the holder a demonlist score");

    let full: FullDemon = clnt
        .get(format!("/api/v2/demons/{}/", demon_id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;
    clnt.patch(format!("/api/v2/demons/{}/", demon_id), &serde_json::json!({"requirement": 60}))
        .authorize_as(&helper)
        .header("If-Match", full.etag_string())
        .expect_status(Status::Ok)
        .execute()
        .await;

    let after: FullPlayer = clnt
        .get(format!("/api/v1/players/{}/", holder.id))
        .expect_status(Status::Ok)
        .get_success_result()
        .await;
    assert_eq!(
        after.player.rated_score, 0.0f64,
        "raising the requirement deleted the holder's only record, so their demonlist score must drop to 0"
    );
    assert_eq!(
        after.player.score, 0.0f64,
        "raising the requirement deleted the holder's only record, so their rated+ score must drop to 0"
    );
}

async fn utc_now(connection: &mut PgConnection) -> sqlx::types::chrono::NaiveDateTime {
    sqlx::query!(r#"SELECT (NOW() AT TIME ZONE 'utc') AS "now!""#)
        .fetch_one(connection)
        .await
        .unwrap()
        .now
}

async fn add_demons(prefix: &str, positions: std::ops::RangeInclusive<i16>, rated: bool, player: i32, connection: &mut PgConnection) {
    for position in positions {
        add_demon(
            format!("{} {}", prefix, position),
            position,
            50,
            player,
            player,
            rated,
            connection,
        )
        .await;
    }
}

async fn setup_demon_legacy_on_ratedplus_only(connection: &mut PgConnection) -> i32 {
    use pointercrate_demonlist::config;
    use pointercrate_demonlist::demon::MinimalDemon;

    let verifier = DatabasePlayer::by_name_or_create("stardust1971", connection).await.unwrap();

    add_demons("Unrated", 1..=100, false, verifier.id, connection).await;
    add_demons("Rated", 101..=199, true, verifier.id, connection).await;
    let target = add_demon("Target", 200, 50, verifier.id, verifier.id, true, connection).await;

    recompute_rated_positions(connection).await.unwrap();

    let target_demon = MinimalDemon::by_id(target, connection).await.unwrap();
    assert!(
        target_demon.position > config::extended_list_size(),
        "sanity: the target demon must be legacy on rated+"
    );
    assert!(
        target_demon.rated_position.unwrap() <= config::extended_list_size(),
        "sanity: the target demon must not be legacy on the demonlist"
    );
    assert!(
        target_demon.rated_position.unwrap() > config::list_size(),
        "sanity: the target demon must not be on the demonlist's main list"
    );

    target
}

#[sqlx::test(migrations = "../migrations")]
async fn test_submit_non100_accepted_on_demon_main_on_demonlist_but_extended_on_ratedplus(pool: Pool<Postgres>) {
    use pointercrate_demonlist::config;
    use pointercrate_demonlist::demon::MinimalDemon;

    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    add_demons("Unrated", 1..=60, false, verifier.id, &mut connection).await;
    add_demons("Rated", 61..=99, true, verifier.id, &mut connection).await;
    let target = add_demon("Target", 100, 50, verifier.id, verifier.id, true, &mut connection).await;

    recompute_rated_positions(&mut connection).await.unwrap();

    let target_demon = MinimalDemon::by_id(target, &mut connection).await.unwrap();
    assert!(
        target_demon.position > config::list_size(),
        "sanity: the target demon must be extended list on rated+"
    );
    assert!(
        target_demon.rated_position.unwrap() <= config::list_size(),
        "sanity: the target demon must be main list on the demonlist"
    );

    let body = submit_as(&clnt, &helper, target, 50, Status::Ok).await;
    assert_eq!(body["data"]["progress"].as_i64(), Some(50), "expected the record to be created, got {}", body);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_submit_100_accepted_on_demon_extended_on_demonlist_but_legacy_on_ratedplus(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let target = setup_demon_legacy_on_ratedplus_only(&mut connection).await;

    let body = submit_as(&clnt, &helper, target, 100, Status::Ok).await;
    assert_eq!(body["data"]["progress"].as_i64(), Some(100), "expected the record to be created, got {}", body);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_submit_non100_rejected_on_demon_extended_on_demonlist_but_legacy_on_ratedplus(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let target = setup_demon_legacy_on_ratedplus_only(&mut connection).await;

    let body = submit_as(&clnt, &helper, target, 50, Status::UnprocessableEntity).await;
    assert_eq!(body["code"].as_i64(), Some(42220), "expected Non100Extended, got {}", body);
}

async fn movement_log_for(clnt: &TestClient, list: &str, demon_id: i32) -> Vec<serde_json::Value> {
    clnt.get(format!("/api/v2/demons/{}/audit/movement/?list={}", demon_id, list))
        .expect_status(Status::Ok)
        .get_result()
        .await
}

fn movement_reasons(log: &[serde_json::Value]) -> Vec<String> {
    log.iter()
        .map(|entry| match &entry["reason"] {
            serde_json::Value::String(reason) => reason.clone(),
            serde_json::Value::Object(reason) => reason.keys().next().unwrap().clone(),
            other => panic!("unexpected movement reason: {}", other),
        })
        .collect()
}

#[sqlx::test(migrations = "../migrations")]
async fn test_movement_log_reports_unrate_and_rerate_on_demonlist(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let demon = clnt
        .add_demon(&helper, "Bloodbath", 1, 100, "stardust1971", "stardust1971", true)
        .await;
    let demon_id = demon.demon.base.id;

    set_demon_rated(&clnt, &helper, demon_id, false).await;
    set_demon_rated(&clnt, &helper, demon_id, true).await;

    let log = movement_log_for(&clnt, "demonlist", demon_id).await;

    assert_eq!(movement_reasons(&log), vec!["Added", "Unrated", "Rated"], "log was {:?}", log);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_movement_log_ignores_rate_changes_on_ratedplus(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let demon = clnt
        .add_demon(&helper, "Bloodbath", 1, 100, "stardust1971", "stardust1971", true)
        .await;
    let demon_id = demon.demon.base.id;

    set_demon_rated(&clnt, &helper, demon_id, false).await;
    set_demon_rated(&clnt, &helper, demon_id, true).await;

    let log = movement_log_for(&clnt, "ratedplus", demon_id).await;

    assert_eq!(movement_reasons(&log), vec!["Added"], "log was {:?}", log);
    assert_eq!(log[0]["new_position"].as_i64(), Some(1), "log was {:?}", log);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_movement_log_unavailable_for_unrated_demon_on_demonlist(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let demon = clnt
        .add_demon(&helper, "Bloodbath", 1, 100, "stardust1971", "stardust1971", true)
        .await;
    let demon_id = demon.demon.base.id;

    set_demon_rated(&clnt, &helper, demon_id, false).await;

    clnt.get(format!("/api/v2/demons/{}/audit/movement/?list=demonlist", demon_id))
        .expect_status(Status::NotFound)
        .execute()
        .await;

    clnt.get(format!("/api/v2/demons/{}/audit/movement/?list=ratedplus", demon_id))
        .expect_status(Status::Ok)
        .execute()
        .await;
}

#[sqlx::test(migrations = "../migrations")]
async fn test_movement_log_reports_other_unrated_on_demonlist(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let player = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    let above = add_demon("Bloodbath", 1, 100, player.id, player.id, true, &mut connection).await;
    let below = clnt
        .add_demon(&helper, "Bloodlust", 2, 100, "stardust1971", "stardust1971", true)
        .await;

    set_demon_rated(&clnt, &helper, above, false).await;

    let log = movement_log_for(&clnt, "demonlist", below.demon.base.id).await;

    assert_eq!(movement_reasons(&log), vec!["Added", "OtherUnrated"], "log was {:?}", log);
    assert_eq!(
        log[1]["reason"]["OtherUnrated"]["other"]["name"].as_str(),
        Some("Bloodbath"),
        "log was {:?}",
        log
    );
    assert_eq!(log[0]["new_position"].as_i64(), Some(2), "log was {:?}", log);
    assert_eq!(log[1]["new_position"].as_i64(), Some(1), "log was {:?}", log);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_movement_log_reports_other_rated_on_demonlist(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let player = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    let above = add_demon("Bloodbath", 1, 100, player.id, player.id, false, &mut connection).await;
    let below = clnt
        .add_demon(&helper, "Bloodlust", 2, 100, "stardust1971", "stardust1971", true)
        .await;

    set_demon_rated(&clnt, &helper, above, true).await;

    let log = movement_log_for(&clnt, "demonlist", below.demon.base.id).await;

    assert_eq!(movement_reasons(&log), vec!["Added", "OtherRated"], "log was {:?}", log);
    assert_eq!(
        log[1]["reason"]["OtherRated"]["other"]["name"].as_str(),
        Some("Bloodbath"),
        "log was {:?}",
        log
    );
    assert_eq!(log[0]["new_position"].as_i64(), Some(1), "log was {:?}", log);
    assert_eq!(log[1]["new_position"].as_i64(), Some(2), "log was {:?}", log);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_time_machine_excludes_demon_that_was_unrated_at_that_time(pool: Pool<Postgres>) {
    use pointercrate_demonlist::demon::list_at;
    use pointercrate_demonlist::list::List;

    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let demon = clnt
        .add_demon(&helper, "Bloodbath", 1, 100, "stardust1971", "stardust1971", false)
        .await;

    let before = utc_now(&mut connection).await;

    set_demon_rated(&clnt, &helper, demon.demon.base.id, true).await;

    let demonlist = list_at(&mut connection, &List::Demonlist, before).await.unwrap();
    assert!(
        demonlist.is_empty(),
        "the demon was unrated at that time, so the demonlist time machine must not contain it, got {:?}",
        demonlist.iter().map(|d| &d.current_demon.base.name).collect::<Vec<_>>()
    );

    let ratedplus = list_at(&mut connection, &List::RatedPlus, before).await.unwrap();
    assert_eq!(ratedplus.len(), 1, "the rated+ time machine must contain the demon regardless");
}

#[sqlx::test(migrations = "../migrations")]
async fn test_time_machine_excludes_demons_added_after_destination(pool: Pool<Postgres>) {
    use pointercrate_demonlist::demon::list_at;
    use pointercrate_demonlist::list::List;

    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let player = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    add_demon("Bloodbath", 1, 100, player.id, player.id, true, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    let before = utc_now(&mut connection).await;

    clnt.add_demon(&helper, "Bloodlust", 1, 100, "stardust1971", "stardust1971", true)
        .await;

    for list in [List::Demonlist, List::RatedPlus] {
        let then = list_at(&mut connection, &list, before).await.unwrap();

        assert_eq!(
            then.iter()
                .map(|d| (d.current_demon.base.name.clone(), d.current_demon.base.position))
                .collect::<Vec<_>>(),
            vec![("Bloodbath".to_string(), 1)],
            "unexpected {} time machine contents",
            list.as_str()
        );
    }
}

#[sqlx::test(migrations = "../migrations")]
async fn test_time_machine_reports_no_current_position_for_since_unrated_demon(pool: Pool<Postgres>) {
    use pointercrate_demonlist::demon::list_at;
    use pointercrate_demonlist::list::List;

    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let player = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    let above = add_demon("Bloodbath", 1, 100, player.id, player.id, true, &mut connection).await;
    add_demon("Bloodlust", 2, 100, player.id, player.id, true, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    let before = utc_now(&mut connection).await;

    set_demon_rated(&clnt, &helper, above, false).await;

    let demonlist = list_at(&mut connection, &List::Demonlist, before).await.unwrap();

    let unrated = demonlist
        .iter()
        .find(|d| d.current_demon.base.name == "Bloodbath")
        .expect("the demonlist time machine must still contain the since-unrated demon");
    assert_eq!(unrated.position_now, None);

    let still_rated = demonlist
        .iter()
        .find(|d| d.current_demon.base.name == "Bloodlust")
        .expect("the demonlist time machine must contain the still-rated demon");
    assert_eq!(still_rated.position_now, Some(1));

    let ratedplus = list_at(&mut connection, &List::RatedPlus, before).await.unwrap();

    let unrated = ratedplus.iter().find(|d| d.current_demon.base.name == "Bloodbath").unwrap();
    assert_eq!(unrated.position_now, Some(1));
}

#[sqlx::test(migrations = "../migrations")]
async fn test_permalink_redirects_to_each_lists_own_position(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let player = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    add_demon("Bloodlust", 1, 100, player.id, player.id, false, &mut connection).await;
    let rated = add_demon("Bloodbath", 2, 100, player.id, player.id, true, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    let response = clnt
        .get(format!("/ratedplus/permalink/{}/", rated))
        .expect_status(Status::SeeOther)
        .execute()
        .await;
    assert_eq!(response.headers().get_one("Location"), Some("/ratedplus/2"));

    let response = clnt
        .get(format!("/demonlist/permalink/{}/", rated))
        .expect_status(Status::SeeOther)
        .execute()
        .await;
    assert_eq!(response.headers().get_one("Location"), Some("/demonlist/1"));
}

#[sqlx::test(migrations = "../migrations")]
async fn test_permalink_not_found_for_unrated_demon_on_demonlist(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let player = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    let unrated = add_demon("Bloodlust", 1, 100, player.id, player.id, false, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    clnt.get(format!("/demonlist/permalink/{}/", unrated))
        .expect_status(Status::NotFound)
        .execute()
        .await;

    clnt.get(format!("/ratedplus/permalink/{}/", unrated))
        .expect_status(Status::SeeOther)
        .execute()
        .await;
}

#[sqlx::test(migrations = "../migrations")]
async fn test_demon_by_position_resolves_per_list(pool: Pool<Postgres>) {
    use pointercrate_demonlist::list::List;

    let (_clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let player = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    add_demon("Bloodlust", 1, 100, player.id, player.id, false, &mut connection).await;
    add_demon("Bloodbath", 2, 100, player.id, player.id, true, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    let on_ratedplus = FullDemon::by_position(1, &List::RatedPlus, &mut connection).await.unwrap();
    assert_eq!(on_ratedplus.demon.base.name, "Bloodlust");

    let on_demonlist = FullDemon::by_position(1, &List::Demonlist, &mut connection).await.unwrap();
    assert_eq!(on_demonlist.demon.base.name, "Bloodbath");

    assert!(
        FullDemon::by_position(2, &List::Demonlist, &mut connection).await.is_err(),
        "the demonlist only has one demon, so position 2 must not resolve"
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn test_player_payload_exposes_per_list_scores_ranks_and_demon_positions(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let verifier_player = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();
    let holder = DatabasePlayer::by_name_or_create("stardust1972", &mut connection).await.unwrap();

    add_demon("Bloodbath", 1, 100, verifier_player.id, verifier_player.id, true, &mut connection).await;
    let unrated = add_demon("Bloodlust", 2, 100, verifier_player.id, verifier_player.id, false, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    add_simple_record(100, holder.id, unrated, RecordStatus::Approved, &mut connection).await;
    recompute_scores(&mut connection).await.unwrap();

    let verifier: serde_json::Value = clnt
        .get(format!("/api/v1/players/{}/", verifier_player.id))
        .expect_status(Status::Ok)
        .get_result()
        .await;
    let verifier = &verifier["data"];

    assert!(
        verifier["score"].as_f64().unwrap() > verifier["rated_score"].as_f64().unwrap(),
        "the verifier of both demons must have a higher rated+ score than demonlist score, got {}",
        verifier
    );
    assert!(verifier["rank"].as_i64().is_some(), "missing rated+ rank in {}", verifier);
    assert!(verifier["rated_rank"].as_i64().is_some(), "missing demonlist rank in {}", verifier);

    let verified = verifier["verified"].as_array().unwrap();
    let rated_entry = verified.iter().find(|demon| demon["name"] == "Bloodbath").unwrap();
    let unrated_entry = verified.iter().find(|demon| demon["name"] == "Bloodlust").unwrap();

    assert_eq!(rated_entry["rated_position"].as_i64(), Some(1), "got {}", rated_entry);
    assert!(unrated_entry["rated_position"].is_null(), "got {}", unrated_entry);

    let holder: serde_json::Value = clnt
        .get(format!("/api/v1/players/{}/", holder.id))
        .expect_status(Status::Ok)
        .get_result()
        .await;
    let record = &holder["data"]["records"][0];

    assert_eq!(record["demon"]["position"].as_i64(), Some(2), "got {}", record);
    assert!(record["demon"]["rated_position"].is_null(), "got {}", record);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_ranking_payload_exposes_per_list_score_and_rank(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();
    let holder = DatabasePlayer::by_name_or_create("holder", &mut connection).await.unwrap();

    add_demon("Bloodlust", 1, 100, verifier.id, verifier.id, false, &mut connection).await;
    let rated = add_demon("Bloodbath", 2, 100, verifier.id, verifier.id, true, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();

    add_simple_record(100, holder.id, rated, RecordStatus::Approved, &mut connection).await;
    recompute_scores(&mut connection).await.unwrap();

    let (demonlist_ranking, _) = clnt
        .get("/api/v1/players/ranking/?list=demonlist")
        .get_pagination_result::<serde_json::Value>()
        .await;
    let (ratedplus_ranking, _) = clnt
        .get("/api/v1/players/ranking/?list=ratedplus")
        .get_pagination_result::<serde_json::Value>()
        .await;

    let on_demonlist = demonlist_ranking.iter().find(|entry| entry["name"] == "holder").unwrap();
    let on_ratedplus = ratedplus_ranking.iter().find(|entry| entry["name"] == "holder").unwrap();

    assert!(on_demonlist["rank"].as_i64().is_some(), "missing rank in {}", on_demonlist);
    assert!(on_ratedplus["rank"].as_i64().is_some(), "missing rank in {}", on_ratedplus);

    assert!(
        on_demonlist["score"].as_f64().unwrap() > on_ratedplus["score"].as_f64().unwrap(),
        "the demon is #1 on the demonlist but #2 on rated+, so the demonlist score must be higher, got {} and {}",
        on_demonlist,
        on_ratedplus
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn test_nation_payload_exposes_rated_positions(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();

    add_demon("Bloodbath", 1, 100, verifier.id, verifier.id, true, &mut connection).await;
    add_demon("Bloodlust", 2, 100, verifier.id, verifier.id, false, &mut connection).await;
    recompute_rated_positions(&mut connection).await.unwrap();
    recompute_scores(&mut connection).await.unwrap();

    clnt.patch_player(verifier.id, &helper, serde_json::json!({"nationality": "GB"}))
        .await
        .execute()
        .await;

    let nation: serde_json::Value = clnt
        .get("/api/v1/nationalities/GB/")
        .expect_status(Status::Ok)
        .get_result()
        .await;
    let verified = nation["data"]["verified"].as_array().unwrap();

    let rated_entry = verified.iter().find(|entry| entry["demon"]["name"] == "Bloodbath").unwrap();
    let unrated_entry = verified.iter().find(|entry| entry["demon"]["name"] == "Bloodlust").unwrap();

    assert_eq!(rated_entry["demon"]["rated_position"].as_i64(), Some(1), "got {}", rated_entry);
    assert!(unrated_entry["demon"]["rated_position"].is_null(), "got {}", unrated_entry);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_unrated_demon_gives_subdivision_only_ratedplus_score(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let demon = clnt
        .add_demon(&helper, "Bloodbath", 1, 100, "stardust1971", "stardust1971", false)
        .await;

    clnt.patch_player(
        demon.demon.verifier.id,
        &helper,
        serde_json::json!({"nationality": "GB", "subdivision": "ENG"}),
    )
    .await
    .execute()
    .await;

    let scores = sqlx::query!("SELECT score, ratedplus_score FROM subdivisions WHERE nation = 'GB' AND iso_code = 'ENG'")
        .fetch_one(&mut *connection)
        .await
        .unwrap();

    assert_eq!(scores.score, 0.0f64, "unrated demon gave the subdivision a demonlist score");
    assert_ne!(scores.ratedplus_score, 0.0f64, "unrated demon failed to give the subdivision a rated+ score");
}

#[sqlx::test(migrations = "../migrations")]
async fn test_heatmap_css_includes_subdivision_rules_per_list(pool: Pool<Postgres>) {
    let (clnt, mut connection) = pointercrate_test::demonlist::setup_rocket(pool).await;

    let helper = pointercrate_test::user::system_user_with_perms(LIST_MODERATOR, &mut connection).await;
    let rated_verifier = DatabasePlayer::by_name_or_create("stardust1971", &mut connection).await.unwrap();
    let unrated_verifier = DatabasePlayer::by_name_or_create("stardust1972", &mut connection).await.unwrap();

    add_demon("Bloodbath", 1, 100, rated_verifier.id, rated_verifier.id, true, &mut connection).await;
    add_demon(
        "Bloodlust",
        2,
        100,
        unrated_verifier.id,
        unrated_verifier.id,
        false,
        &mut connection,
    )
    .await;
    recompute_rated_positions(&mut connection).await.unwrap();
    recompute_scores(&mut connection).await.unwrap();

    clnt.patch_player(
        rated_verifier.id,
        &helper,
        serde_json::json!({"nationality": "GB", "subdivision": "SCT"}),
    )
    .await
    .execute()
    .await;
    clnt.patch_player(
        unrated_verifier.id,
        &helper,
        serde_json::json!({"nationality": "GB", "subdivision": "ENG"}),
    )
    .await
    .execute()
    .await;

    let demonlist_css = clnt
        .get("/demonlist/statsviewer/heatmap.css")
        .expect_status(Status::Ok)
        .execute()
        .await
        .into_string()
        .await
        .unwrap();
    let ratedplus_css = clnt
        .get("/ratedplus/statsviewer/heatmap.css")
        .expect_status(Status::Ok)
        .execute()
        .await
        .into_string()
        .await
        .unwrap();

    assert!(
        demonlist_css.contains("#GB-SCT"),
        "Demonlist heatmap missing scored subdivision: {}",
        demonlist_css
    );
    assert!(
        !demonlist_css.contains("#GB-ENG"),
        "Demonlist heatmap contains a subdivision scored only by an unrated demon: {}",
        demonlist_css
    );

    assert!(
        ratedplus_css.contains("#GB-SCT"),
        "Rated+ heatmap missing scored subdivision: {}",
        ratedplus_css
    );
    assert!(
        ratedplus_css.contains("#GB-ENG"),
        "Rated+ heatmap missing subdivision scored by an unrated demon: {}",
        ratedplus_css
    );
}
