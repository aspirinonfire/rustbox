use std::{collections::HashSet, error::Error};

use bson::{doc, oid::ObjectId};
use futures_util::TryStreamExt;
use mongodb::{bson::DateTime, options::IndexOptions, Collection, Database, IndexModel};
use serde::{self, Deserialize, Serialize};

#[allow(dead_code)]
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

#[allow(dead_code)]
impl Player {
    pub fn get_player_collection(mongo_database: &Database) -> Collection<Player> {
        mongo_database.collection::<Player>("players")
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
    pub async fn get_player_by_existing_identity(
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
