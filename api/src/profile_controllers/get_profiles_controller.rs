use crate::database::DB;
use entity::profile;
use rocket::serde::json::{json, Value};
use sea_orm::{EntityTrait, QueryOrder};
use sea_orm_rocket::Connection;
use serde::Serialize;

use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;

#[derive(Serialize, JsonSchema)]
struct MappedGetProfiles {
    id: i32,
    name: String,
    email: String,
    created_at: String,
}

#[openapi(tag = "Profiles")]
#[get("/", format = "json")]
pub async fn run(connection: Connection<'_, DB>) -> Value {
    let db = connection.into_inner();

    let db_profiles = profile::Entity::find()
        .order_by_desc(profile::Column::CreatedAt)
        .all(db)
        .await;

    let profiles = match db_profiles {
        Ok(profiles) => profiles,
        Err(_) => Vec::new(),
    };

    let mapped_profiles = profiles
        .iter()
        .map(|profile| MappedGetProfiles {
            id: profile.id,
            name: profile.name.to_owned(),
            email: profile.email.to_owned(),
            created_at: profile.created_at.to_string(),
        })
        .collect::<Vec<MappedGetProfiles>>();

    json!({ "profiles": mapped_profiles })
}
