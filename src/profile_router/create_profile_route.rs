use crate::response_structs::SuccessResponse;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateProfileRequest {
    message: String,
}

#[derive(Serialize)]
pub struct CreateProfileResponse {
    data: String,
}

#[post("/", format = "json", data = "<request>")]
pub fn run(
    request: Json<CreateProfileRequest>,
) -> Json<SuccessResponse<CreateProfileResponse>> {
    let response = CreateProfileResponse {
        data: request.message.to_string() + " from request",
    };

    Json(SuccessResponse {
        data: response,
        code: 200,
    })
}
