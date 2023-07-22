mod hello_rocket_view;

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("VIEWS", |rocket| async {
        rocket.mount("/", routes![hello_rocket_view::index])
    })
}
