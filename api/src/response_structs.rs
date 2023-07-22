use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub data: T,
    pub code: u32,
}
