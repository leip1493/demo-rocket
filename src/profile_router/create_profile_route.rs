use crate::database::DB;
use crate::response_structs::SuccessResponse;
use entity::profile;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use sea_orm_rocket::Connection;

#[derive(Debug, Serialize)]
struct Profile {
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub struct CreateProfileRequest {
    name: String,
    email: String,
}

#[derive(Serialize)]
pub struct CreateProfileResponse {
    profile: Profile,
}

#[post("/", format = "json", data = "<request>")]
pub async fn run(
    connection: Connection<'_, DB>, // <- Recibe la instancia DB desde Rocket
    request: Json<CreateProfileRequest>,
) -> Result<Json<SuccessResponse<CreateProfileResponse>>, Value> {
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
        Err(e) => return Err(json!({ "message": format!("[SAVING] {}", e.to_string())})),
    };

    let response = CreateProfileResponse { profile: profile };

    Ok(Json(SuccessResponse {
        data: response,
        code: 200,
    }))
}
