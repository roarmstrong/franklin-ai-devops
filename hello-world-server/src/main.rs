use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            // We allow the visitor to see an index of the images at `/images`.
            .service(Files::new("/images", "static/images/").show_files_listing())
            // Serve a tree of static files at the web root and specify the index file.
            // Note that the root path should always be defined as the last item. The paths are
            // resolved in the order they are defined. If this would be placed before the `/images`
            // path then the service for the static images would never be reached.
            .service(Files::new("/", "./static/root/").index_file("index.html"))
            // Enable the logger.
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
