#[macro_use]
extern crate rocket;
use std::path::PathBuf;

use rocket::response::status::NotFound;
use rocket::serde::json::{json, Json, Value};

use rocket_dyn_templates::Template;
use sea_orm_rocket::Database;
use serde::Serialize;

mod database;
use database::DB;

mod profile_router;
mod response_structs;
mod views;

#[derive(Serialize)]
struct Resp {
    lorem: String,
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn get_templates_dir() -> PathBuf {
    // Obtener la ruta del directorio actual en tiempo de ejecución
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    // Construir la ruta completa al directorio de plantillas
    return current_dir.join("api").join("templates");
}

#[rocket::main]
async fn start() -> Result<(), rocket::Error> {
    let figment = rocket::Config::figment().merge(("template_dir", get_templates_dir()));

    rocket::custom(figment)
        .attach(DB::init())
        .attach(views::stage())
        .attach(profile_router::stage())
        .attach(Template::fairing())        
        .register("/", catchers![not_found])
        .launch()
        .await
        .map(|_| ())
}

pub fn main() {
    let result = start();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
