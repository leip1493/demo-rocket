use crate::response_structs::SuccessResponse;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
pub(crate) struct GetProfileResponse {
    pub(crate) message: String,
}

#[get("/")]
pub(crate) fn run() -> Json<SuccessResponse<GetProfileResponse>> {
    let response = GetProfileResponse {
        message: "lorem ipsum".to_string(),
    };

    Json(SuccessResponse {
        data: response,
        code: 200,
    })
}
