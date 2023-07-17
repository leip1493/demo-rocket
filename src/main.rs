#[macro_use]
extern crate rocket;
use rocket::http::hyper::server::conn;
use rocket::http::Status;
use rocket::response::status::{self, NotFound};
use rocket::serde::json::{json, Error, Json, Value};

use rocket_dyn_templates::{context, Template};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use serde::Serialize;
use std::time::Duration;

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
    let db = connect_db().await;

    let connection = match db {
        Ok(connection) => println!("Conectado {:?}", connection),
        Err(e) => return Some(json!({ "message": e.to_string()})),
    };

    // println!("Database: {:?}", db);

    let message = "ipsums";

    if message.ne("ipsum") {
        return None;
    }

    Some(json!({ "message": message}))
}

async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    let mut opt =
        ConnectOptions::new("postgres://postgres:secret@localhost:5432/hello_rocket".to_owned());

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let db = Database::connect(opt).await?;

    Ok(db)

    // let connection = match db {
    //     Ok(db) => db,
    //     Err(err) => {
    //         return Err(ServerError {
    //             message: err.to_string(),
    //         })
    //     }
    // };

    // Ok(connection)
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, about, lorem])
        .register("/", catchers![not_found])
        .attach(profile_router::stage())
        .attach(Template::fairing())
}
