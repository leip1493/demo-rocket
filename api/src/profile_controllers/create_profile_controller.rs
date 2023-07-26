use crate::database::DB;
use entity::profile;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use sea_orm_rocket::Connection;

use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;

#[derive(Debug, Serialize, JsonSchema)]
pub struct Profile {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct CreateProfileRequest {
    name: String,
    email: String,
}

#[derive(Serialize, JsonSchema)]
pub struct CreateProfileError {
    message: String,
}

#[derive(Serialize, JsonSchema)]
pub struct CreateProfileResponse {
    pub profile: Profile,
}

#[openapi(tag = "Profiles")]
#[post("/", format = "json", data = "<request>")]
pub async fn run(
    connection: Connection<'_, DB>, // <- Recibe la instancia DB desde Rocket
    request: Json<CreateProfileRequest>,
) -> Result<Json<CreateProfileResponse>, (Status, Json<CreateProfileError>)> {
    let db = connection.into_inner();

    let profile = Profile {
        name: request.name.to_string(),
        email: request.email.to_string(),
    };

    let db_profile = profile::ActiveModel {
        name: Set(profile.name.to_owned()),
        email: Set(profile.email.to_owned()),
        ..Default::default()
    };

    match db_profile.save(db).await {
        Ok(_) => (),
        Err(e) => {
            return Err((
                Status::InternalServerError,
                Json(CreateProfileError {
                    message: e.to_string(),
                }),
            ));
        }
    };

    Ok(Json(CreateProfileResponse { profile }))
}
