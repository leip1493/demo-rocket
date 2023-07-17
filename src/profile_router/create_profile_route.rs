use crate::{database, response_structs::SuccessResponse};
use entity::profile;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use std::fmt;

#[derive(Debug, Serialize)]
struct Profile {
    name: String,
    email: String,
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "El correo de: {} es {}", self.name, self.email)
    }
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
    request: Json<CreateProfileRequest>,
) -> Result<Json<SuccessResponse<CreateProfileResponse>>, Value> {
    let db = match database::connect_db().await {
        Ok(connection) => connection,
        Err(e) => return Err(json!({ "message": format!("[CONNECTING] {}", e.to_string())})),
    };

    let profile = Profile {
        name: request.name.to_string(),
        email: request.email.to_string(),
    };

    println!("{:?}", profile);
    println!("{}", profile);

    let db_profile = profile::ActiveModel {
        name: Set(profile.name.to_owned()),
        email: Set(profile.email.to_owned()),
        ..Default::default()
    };

    match db_profile.save(&db).await {
        Ok(_) => (),
        Err(e) => return Err(json!({ "message": format!("[SAVING] {}", e.to_string())})),
    };

    let response = CreateProfileResponse { profile: profile };

    Ok(Json(SuccessResponse {
        data: response,
        code: 200,
    }))
}
