use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct SuccessResponse<T> {
    data: T,
    code: u32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GetProfileResponse {
    message: String,
}

#[get("/")]
fn profile() -> Json<SuccessResponse<GetProfileResponse>> {
    let response = GetProfileResponse {
        message: "lorem ipsum".to_string(),
    };

    Json(SuccessResponse {
        data: response,
        code: 200,
    })
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct CreateProfileResponse {
    data: String,
}

#[post("/")]
fn create_profile() -> Json<SuccessResponse<CreateProfileResponse>> {
    let response = CreateProfileResponse {
        data: "foo bar".to_string(),
    };

    Json(SuccessResponse {
        data: response,
        code: 200,
    })
}

#[put("/")]
fn update_profile() -> &'static str {
    "update profile"
}

#[delete("/")]
fn delete_profile() -> &'static str {
    "delete profile"
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("PROFILE", |rocket| async {
        rocket.mount(
            "/profile",
            routes![profile, create_profile, update_profile, delete_profile,],
        )
    })
}
