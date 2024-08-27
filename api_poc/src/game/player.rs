use std::{collections::HashSet, error::Error};

use bson::{doc, oid::ObjectId};
use futures_util::TryStreamExt;
use mongodb::{bson::DateTime, options::IndexOptions, Collection, Database, IndexModel};
use serde::{self, Deserialize, Serialize};

const PLAYER_COLL_NAME: &str = "players";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Player {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    pub name: String,
    pub date_created: DateTime,

    #[serde(default)]
    pub games_owned: HashSet<ObjectId>,

    #[serde(default)]
    pub games_invited: HashSet<ObjectId>,

    /// OAuth provder - Google, Apple, etc
    /// For the time being, we wont be opening a can of worms called merged accounts, so for now unique login == new player
    pub provider_name: String,
    /// Identity unique id (in the context of Provider above)
    pub provider_identity_id: String,
    /// Game API refresh token
    pub api_refresh_token: String,
    /// Game API refresh token expiration
    pub api_refresh_token_exp: DateTime,
}

impl Player {
    fn get_player_collection(mongo_database: &Database) -> Collection<Player> {
        mongo_database.collection::<Player>(PLAYER_COLL_NAME)
    }

    pub async fn create_identity_index(
        mongo_database: &Database,
    ) -> mongodb::results::CreateIndexResult {
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! { "provider_name": 1, "provider_identity_id": 1 })
            .options(options)
            .build();

        // creating index is critical to api operations therefore we panic on errors
        Self::get_player_collection(mongo_database)
            .create_index(model)
            .await
            .expect("creating index collection should succeed")
    }

    /// Retrieve existing player using identity
    async fn get_player_by_existing_identity(
        mongo_database: &Database,
        provider_name: &str,
        provider_identity_id: &str,
        // using Box dyn because we may get back mongo error or "business" error - duplicate player
    ) -> Result<Option<Player>, Box<dyn Error>> {
        let player_filter =
            doc! { "provider_name": provider_name, "provider_identity_id": provider_identity_id };

        let mut cursor = Self::get_player_collection(mongo_database)
            .find(player_filter)
            .await?;

        let first_player = cursor.try_next().await?;

        if cursor.try_next().await?.is_some() {
            return Err("more than one matching player found!".into());
        }

        Ok(first_player)
    }

    pub async fn create_from_external_identity(
        mongo_database: &Database,
        name: &str,
        provider_name: &str,
        provider_identity_id: &str,
        api_refresh_token: &str,
        api_refresh_token_exp: DateTime,
    ) -> Result<Self, Box<dyn Error>> {
        // validate for existing players
        let existing_player = Self::get_player_by_existing_identity(
            mongo_database,
            provider_name,
            provider_identity_id,
        )
        .await?;

        if existing_player.is_some() {
            return Err("player with supplied identity already exist!".into());
        }

        let new_player = Self {
            id: ObjectId::new(),
            name: name.into(),
            date_created: DateTime::now(),

            provider_name: provider_name.into(),
            provider_identity_id: provider_identity_id.into(),
            api_refresh_token: api_refresh_token.into(),
            api_refresh_token_exp,

            games_owned: HashSet::new(),
            games_invited: HashSet::new(),
        };

        Self::get_player_collection(mongo_database)
            .insert_one(&new_player)
            .await?;

        Ok(new_player)
    }
}

#[cfg(test)]
mod tests {
    use mongodb::Client;

    use super::*;

    use std::time::Duration;

    use testcontainers::{
        core::{IntoContainerPort, WaitFor}, runners::AsyncRunner, ContainerAsync, GenericImage, ImageExt
    };

    pub const TEST_DB_NAME: &str = "test_db";
    const TEST_USERNAME: &str = "root";
    const TEST_PASSWORD: &str = "root";
    const DEFAULT_MONGO_PORT: u16 = 27017;

    pub async fn get_mongo_client_with_container() -> (ContainerAsync<GenericImage>, Client) {
        let container = GenericImage::new("mongo", "7.0")
            .with_wait_for(WaitFor::message_on_stdout("mongod startup complete"))
            .with_exposed_port(DEFAULT_MONGO_PORT.tcp())
            // configure mongo container
            .with_env_var("MONGO_INITDB_DATABASE", TEST_DB_NAME)
            .with_env_var("MONGO_INITDB_ROOT_USERNAME", TEST_USERNAME)
            .with_env_var("MONGO_INITDB_ROOT_PASSWORD", TEST_PASSWORD)
            // startup and network config
            .with_startup_timeout(Duration::from_secs(30))
            .start()
            .await
            .expect("Mongo container must be started");

        let dynamic_port = &container.ports()
            .await
            .expect("Mongo container must have ports")
            .map_to_host_port_ipv4(DEFAULT_MONGO_PORT.tcp())
            .expect("mongo container must have 27017 exposed");

        let conn_string = format!("mongodb://{TEST_USERNAME}:{TEST_PASSWORD}@localhost:{dynamic_port}/{TEST_DB_NAME}?directConnection=true&authSource=admin");

        let mongo_client = Client::with_uri_str(conn_string)
            .await
            .expect("Mongo client must be created");


        (container, mongo_client)
    }

    #[actix_web::test]
    async fn will_return_player_when_exists() -> Result<(), Box<dyn std::error::Error + 'static>> {
        // _container must be captured in the variable so the teardown wont happen too soon
        let (_container, mongo_client) = get_mongo_client_with_container().await;

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

        let actual_player = Player::get_player_by_existing_identity(
            &game_db,
            "provider_name",
            "provider_identity_id",
        )
        .await?
        .expect("test player must be present");

        assert_eq!(test_player, actual_player);

        Ok(())
    }

    #[actix_web::test]
    async fn will_create_new_player() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let (_container, mongo_client) = get_mongo_client_with_container().await;

        let game_db = mongo_client.database(TEST_DB_NAME);

        Player::create_identity_index(&game_db).await;

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
}
