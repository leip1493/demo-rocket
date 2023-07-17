use std::fmt;

use crate::response_structs::SuccessResponse;
use rocket::serde::{json::Json, Deserialize, Serialize};

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
    data: Profile,
}

#[post("/", format = "json", data = "<request>")]
pub fn run(request: Json<CreateProfileRequest>) -> Json<SuccessResponse<CreateProfileResponse>> {
    let profile = Profile {
        name: request.name.to_string(),
        email: request.email.to_string(),
    };

    println!("{:?}", profile);
    println!("{}", profile);

    let response = CreateProfileResponse { data: profile };

    Json(SuccessResponse {
        data: response,
        code: 200,
    })
}
