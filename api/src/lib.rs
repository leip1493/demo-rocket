#[macro_use]
extern crate rocket;
use std::env;
use std::path::PathBuf;

use dotenvy::dotenv;
use rocket::serde::json::{json, Value};
use rocket_dyn_templates::Template;

use rocket_okapi::mount_endpoints_and_merged_docs;
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::{OpenApiSettings, UrlObject};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

use sea_orm_rocket::Database;
use serde::Serialize;

mod database;
use database::DB;

mod cors;
mod openapi_spec;
mod profile_controllers;
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

#[tokio::main]
async fn start() -> Result<(), rocket::Error> {
    dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or(String::from("8000"))
        .parse::<i32>()
        .unwrap();

    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("template_dir", get_templates_dir()));

    let mut building_rocket = rocket::custom(figment)
        .attach(DB::init())
        .attach(views::stage())
        .attach(Template::fairing())
        .mount(
            "/swagger/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("Rocket/SeaOrm - RapiDoc documentation | RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .register("/", catchers![not_found])
        .attach(cors::run(&port));

    let openapi_settings = OpenApiSettings::default();
    let custom_route_spec = (vec![], openapi_spec::run(&port));

    mount_endpoints_and_merged_docs! {
        building_rocket, "/".to_owned(), openapi_settings,
        "/" => custom_route_spec,
        "/profile" => profile_controllers::get_routes_and_docs(&openapi_settings),
    };

    building_rocket.launch().await.map(|_| ())
}

pub fn main() {
    let result = start();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
