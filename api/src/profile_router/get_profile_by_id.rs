use crate::database::DB;
use entity::profile;
use rocket::http::Status;
use rocket::serde::json::{json, Value};
use rocket_okapi::openapi;
use sea_orm::EntityTrait;
use sea_orm_rocket::Connection;

#[openapi(tag = "Profiles")]
#[get("/<id>", format = "json")]
pub async fn run(connection: Connection<'_, DB>, id: i32) -> (Status, Value) {
    let db = connection.into_inner();

    let result_db_profile = profile::Entity::find_by_id(id).one(db).await;
    if let Err(error) = result_db_profile {
        return (
            Status::InternalServerError,
            json!({ "error": error.to_string() }),
        );
    }

    let option_db_profile = result_db_profile.unwrap();
    if let None = option_db_profile {
        return (Status::NotFound, json!({ "profile": () }));
    }

    let db_profile = option_db_profile.unwrap();

    return (Status::Ok, json!({ "profile": db_profile}));
}
