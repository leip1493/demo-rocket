use rocket_okapi::okapi::openapi3::OpenApi;

pub fn run() -> OpenApi {
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
