use crate::database::DB;
use entity::profile;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{json, Value};
use sea_orm::{EntityTrait, ModelTrait};
use sea_orm_rocket::Connection;

#[delete("/<id>")]
pub async fn run(connection: Connection<'_, DB>, id: i32) -> status::Custom<Value> {
    let db = connection.into_inner();

    let result_db_profile = profile::Entity::find_by_id(id).one(db).await;
    if let Err(error) = result_db_profile {
        return status::Custom(
            Status::InternalServerError,
            json!({ "error": error.to_string() }),
        );
    }

    let option_db_profile = result_db_profile.unwrap();
    if let None = option_db_profile {
        return status::Custom(
            Status::NotFound,
            json!({ "error": format!("Profile with id {} not found", id) }),
        );
    }

    let db_profile = option_db_profile.unwrap();

    if let Err(error) = db_profile.delete(db).await {
        return status::Custom(
            Status::InternalServerError,
            json!({ "error": error.to_string() }),
        );
    }

    status::Custom(
        Status::Ok,
        json!({ "message": format!("Profile with id {} deleted", id)}),
    )
}
