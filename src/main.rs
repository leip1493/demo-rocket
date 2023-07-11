#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};

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

#[get("/about")]
fn about() -> &'static str {
    "about"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, about])
        .attach(profile_router::stage())
        .attach(Template::fairing())
}
