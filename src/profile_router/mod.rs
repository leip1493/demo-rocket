mod create_profile_route;
mod delete_profile_route;
mod get_profile_by_id;
mod get_profiles_route;
mod update_profile_route;

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("PROFILE", |rocket| async {
        rocket.mount(
            "/profile",
            routes![
                get_profiles_route::run,
                create_profile_route::run,
                update_profile_route::run,
                get_profile_by_id::run,
                delete_profile_route::run,
            ],
        )
    })
}
