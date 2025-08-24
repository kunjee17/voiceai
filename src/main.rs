use actix_web::{web, App, HttpServer};
use actix_files::Files;
use tera::Tera;
use sqlx::sqlite::SqlitePool;
use tracing::{info, error};

mod handlers;
mod models;
mod services;
mod database;
mod base64;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    info!("Starting Voice AI POC server...");

    // Initialize database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:/workspace/voiceai.db".to_string());
    
    let pool = match SqlitePool::connect(&database_url).await {
        Ok(pool) => {
            info!("Database connected successfully");
            pool
        }
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    // Initialize database tables
    if let Err(e) = database::init_db(&pool).await {
        error!("Failed to initialize database: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
    }

    // Initialize Tera templates
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => {
            info!("Templates loaded successfully");
            t
        }
        Err(e) => {
            error!("Failed to load templates: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    let tera = web::Data::new(tera);
    let pool = web::Data::new(pool);

    info!("Server starting on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(tera.clone())
            .app_data(pool.clone())
            .route("/", web::get().to(handlers::index))
            .route("/api/record", web::post().to(handlers::record_audio))
            .route("/api/recordings", web::get().to(handlers::get_recordings))
            .route("/api/recordings/{id}", web::get().to(handlers::get_recording))
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
