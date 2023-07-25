use rocket_okapi::{okapi::openapi3::OpenApi, openapi_get_routes_spec, settings::OpenApiSettings};

mod create_profile_route;
mod delete_profile_route;
mod get_profile_by_id;
mod get_profiles_route;
mod update_profile_route;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: create_profile_route::run, get_profiles_route::run,get_profile_by_id::run,update_profile_route::run,delete_profile_route::run]
}
