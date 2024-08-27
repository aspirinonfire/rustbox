mod common;
#[path = "../src/game/mod.rs"]
mod game;

use std::collections::HashSet;

use bson::{oid::ObjectId, DateTime};
use game::player::Player;

pub const TEST_DB_NAME: &str = "test_db";

#[actix_web::test]
async fn int_will_return_player_when_exists() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // _container must be captured in the variable so the teardown wont happen too soon
    let (_container, mongo_client) = common::get_mongo_client_with_container().await;

    let game_db = mongo_client.database(TEST_DB_NAME);

    let test_player = Player {
        id: ObjectId::new(),
        name: "name".into(),
        date_created: DateTime::now(),

        provider_name: "provider_name".into(),
        provider_identity_id: "provider_identity_id".into(),
        api_refresh_token: "api_refresh_token".into(),
        api_refresh_token_exp: DateTime::now(),

        games_owned: HashSet::new(),
        games_invited: HashSet::new(),
    };

    let writable_collection = Player::get_player_collection(&game_db);
    writable_collection.insert_one(&test_player).await?;

    let actual_player =
        Player::get_player_by_existing_identity(&game_db, "provider_name", "provider_identity_id")
            .await?
            .expect("test player must be present");

    assert_eq!(test_player, actual_player);

    Ok(())
}

#[actix_web::test]
async fn int_will_create_new_player() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let (_container, mongo_client) = common::get_mongo_client_with_container().await;

    let game_db = mongo_client.database(TEST_DB_NAME);

    Player::create_identity_index(&game_db)
        .await
        .expect("failed to create player index");

    let actual_new_player = Player::create_from_external_identity(
        &game_db,
        "test player",
        "test_provider",
        "test_provider_identity_id",
        "test_api_refresh_token",
        DateTime::now(),
    )
    .await?;

    let actual_player_from_qry = Player::get_player_by_existing_identity(
        &game_db,
        &actual_new_player.provider_name,
        &actual_new_player.provider_identity_id,
    )
    .await?
    .expect("test player must be present");

    assert_eq!(actual_new_player, actual_player_from_qry);

    Ok(())
}
