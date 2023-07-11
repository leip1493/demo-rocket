mod get_profile_route;
mod create_profile_route;
mod update_profile_route;
mod delete_profile_route;

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("PROFILE", |rocket| async {
        rocket.mount(
            "/profile",
            routes![
                get_profile_route::run,
                create_profile_route::run,
                update_profile_route::run,
                delete_profile_route::run,
            ],
        )
    })
}
