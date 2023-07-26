use rocket::http::Method;
use rocket_cors;
use rocket_cors::AllowedHeaders;
use rocket_cors::AllowedOrigins;
use rocket_cors::Cors;

pub fn run() -> Cors {
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
