#[macro_use]
extern crate rocket;
use std::path::PathBuf;

use rocket::http::Method;

use rocket::serde::json::{json, Value};
use rocket_dyn_templates::Template;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};
use rocket_okapi::mount_endpoints_and_merged_docs;
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig};
use rocket_okapi::settings::{OpenApiSettings, UrlObject};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

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

#[tokio::main]
async fn start() -> Result<(), rocket::Error> {
    let figment = rocket::Config::figment().merge(("template_dir", get_templates_dir()));

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
        .attach(cors());

    let openapi_settings = OpenApiSettings::default();
    let custom_route_spec = (vec![], custom_openapi_spec());

    mount_endpoints_and_merged_docs! {
        building_rocket, "/".to_owned(), openapi_settings,
        "/" => custom_route_spec,
        "/profile" => profile_router::get_routes_and_docs(&openapi_settings),
    };

    building_rocket.launch().await.map(|_| ())
}

fn custom_openapi_spec() -> OpenApi {
    use rocket_okapi::okapi::openapi3::*;
    OpenApi {
        openapi: OpenApi::default_version(),
        info: Info {
            title: "SeaOrm-Rocket-Okapi Example".to_owned(),
            description: Some("API Docs for Rocket/SeaOrm example".to_owned()),
            terms_of_service: Some("https://github.com/SeaQL/sea-orm#license".to_owned()),
            contact: Some(Contact {
                name: Some("SeaOrm".to_owned()),
                url: Some("https://github.com/SeaQL/sea-orm".to_owned()),
                email: None,
                ..Default::default()
            }),
            license: Some(License {
                name: "MIT".to_owned(),
                url: Some("https://github.com/SeaQL/sea-orm/blob/master/LICENSE-MIT".to_owned()),
                ..Default::default()
            }),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            ..Default::default()
        },
        servers: vec![
            Server {
                url: "http://127.0.0.1:8000/".to_owned(),
                description: Some("Localhost".to_owned()),
                ..Default::default()
            },
            Server {
                url: "https://production-server.com/".to_owned(),
                description: Some("Remote development server".to_owned()),
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}

fn cors() -> Cors {
    let allowed_origins =
        AllowedOrigins::some_exact(&["http://localhost:8000", "http://127.0.0.1:8000"]);

    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap()
}

pub fn main() {
    let result = start();

    println!("Rocket: deorbit.");

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
