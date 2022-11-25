use std::sync::Mutex;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    get,
    middleware::{DefaultHeaders, Logger},
    web,
    web::Data,
    App, HttpResponse, HttpServer, Responder,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting server");
    log::info!("Server running on http://localhost:8000");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        let svelte_server = actix_files::Files::new("/", "./dist/")
            .index_file("index.html")
            .use_last_modified(true)
            .use_etag(true)
            .default_handler(
                actix_files::NamedFile::open(vec!["./dist/", "index.html"].join("/"))
                    .expect("index file should exist"),
            );

        App::new()
            // .app_data(Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(
                DefaultHeaders::new()
                    .add(("Cross-Origin-Embedder-Policy", "require-corp"))
                    .add(("Cross-Origin-Opener-Policy", "same-origin")),
            )
            // .service(api_scope())
            .service(svelte_server)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
