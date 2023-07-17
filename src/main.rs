#[macro_use]
extern crate rocket;
use rocket::response::status::NotFound;
use rocket::serde::json::{json, Json, Value};

use rocket_dyn_templates::{context, Template};
use serde::Serialize;

mod database;
mod profile_router;
mod response_structs;

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Rocket demo"
        },
    )
}

#[derive(Serialize)]
struct Resp {
    lorem: String,
}

#[get("/about", format = "json")]
async fn about() -> Result<Json<Resp>, NotFound<String>> {
    // let db = connect_db().await?;
    let message = "ipsums";

    if message.ne("ipsum") {
        return Err(NotFound("message invalid".to_string()));
    }

    Ok(Json(Resp {
        lorem: message.to_string(),
    }))
}

#[get("/lorem", format = "json")]
async fn lorem() -> Option<Value> {
    let message = "ipsums";

    if message.ne("ipsum") {
        return None;
    }

    Some(json!({ "message": message}))
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, about, lorem])
        .register("/", catchers![not_found])
        .attach(profile_router::stage())
        .attach(Template::fairing())
}
