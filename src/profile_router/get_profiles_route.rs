use crate::database::DB;
use entity::profile;
use rocket::serde::json::{json, Value};
use sea_orm::{EntityTrait, QueryOrder};
use sea_orm_rocket::Connection;

#[get("/")]
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

    json!({ "profiles": profiles })
}
