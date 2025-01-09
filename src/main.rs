mod auth;
mod config;
mod db;
mod dtos;
mod error;
mod handler;
mod models;
mod utils;
use actix_cors::Cors;
use actix_web::{
    get, http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
use config::Config;
use db::DbClient;
use dotenv::dotenv;
use dtos::{
    FilterUserDto, LoginUserDto, RegisterUserDto, Response, UserDto, UserListResponseDto,
    UserLoginResponseDto, UserResponseDto,
};
use sqlx::postgres::PgPoolOptions;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use handler::{auth as authHandler, users};
#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DbClient,
}

#[derive(OpenApi)]
#[openapi(
    paths(authHandler::login, authHandler::logout, authHandler::register, users::get_me, users::get_users, health_checker_handler),
    components(schemas(
        UserDto,
        FilterUserDto,
        LoginUserDto,
        RegisterUserDto,
        UserResponseDto,
        UserLoginResponseDto,
        Response,
        UserListResponseDto
    )),
    tags(
        (name = "Rust Authentication Api", description = "Authentication in Rust API")
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    openssl_probe::init_ssl_cert_env_vars();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();

    let config = Config::init();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres");

    match sqlx::migrate!().run(&pool).await {
        Ok(_) => {
            println!("Database migration successful");
        }
        Err(e) => {
            eprintln!("Failed to migrate database: {:?}", e);
        }
    }

    let db_client = DbClient::new(pool);
    let app_state = AppState {
        env: config.clone(),
        db_client,
    };

    println!(
        "{}",
        format!("Server running at http://localhost:{}", config.port)
    );

    let open_api = ApiDoc::openapi();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(authHandler::auth_handler())
            .service(users::users_handler())
            .service(health_checker_handler)
            .service(Redoc::with_url("/redoc", open_api.clone()))
            .service(RapiDoc::new("/api-docs/openapi.json").path("/redoc"))
            .service(SwaggerUi::new("/{_:.*}").url("/api-docs/openapi.json", open_api.clone()))
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/api/healthchecker",
    tag = "Health Checker Endpoint",
    responses(
        (status = 200, description= "Authenticated User", body = Response),       
    )
)]
#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Complete Rust API";

    HttpResponse::Ok().json(serde_json::json!({"status":"success", "message": MESSAGE}))
}
