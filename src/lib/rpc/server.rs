use actix_web::{server, App, HttpRequest};
use actix_web::middleware::cors::Cors;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

pub fn start_server() {
    server::new(|| {
        App::new().configure(|app| {
                Cors::for_app(app)
                    // .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .resource("/", |r| r.f(index))
                    .register()
            })
        })
        .bind("127.0.0.1:8545")
        .unwrap()
        .run();
}
