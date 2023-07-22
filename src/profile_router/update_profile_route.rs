use crate::database::DB;
use entity::profile;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, TryIntoModel};
use sea_orm_rocket::Connection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    name: String,
    email: String,
}

#[put("/<id>", format = "json", data = "<request>")]
pub async fn run(
    connection: Connection<'_, DB>,
    id: i32,
    request: Json<UpdateProfileRequest>,
) -> (Status, Value) {
    let db = connection.into_inner();

    let db_profile = profile::Entity::find_by_id(id).one(db).await;

    if let Err(error) = db_profile {
        return (
            Status::InternalServerError,
            json!({ "error": error.to_string() }),
        );
    }

    let option_db_profile = db_profile.unwrap();
    if let None = option_db_profile {
        return (
            Status::NotFound,
            json!({ "error": format!("Profile with id {} not found", id) }),
        );
    }

    let mut active_model_profile: profile::ActiveModel = option_db_profile.unwrap().into();

    active_model_profile.name = Set(request.name.to_owned());
    active_model_profile.email = Set(request.email.to_owned());

    let profile = match active_model_profile.save(db).await {
        Ok(active_model) => active_model.try_into_model(),
        Err(error) => {
            return (
                Status::InternalServerError,
                json!({ "error": error.to_string() }),
            )
        }
    };

    if let Err(error) = &profile {
        return (
            Status::InternalServerError,
            json!({ "error": error.to_string() }),
        );
    }

    let profile = profile.unwrap();

    (Status::Ok, json!({ "profile": profile}))
}
