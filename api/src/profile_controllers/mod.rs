use rocket_okapi::{okapi::openapi3::OpenApi, openapi_get_routes_spec, settings::OpenApiSettings};

mod create_profile_controller;
mod delete_profile_controller;
mod get_profile_by_id_controller;
mod get_profiles_controller;
mod update_profile_controller;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
    settings: create_profile_controller::run,
    get_profiles_controller::run,
    get_profile_by_id_controller::run,
    update_profile_controller::run,
    delete_profile_controller::run
    ]
}
