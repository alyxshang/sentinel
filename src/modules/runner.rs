/*
Sentinel by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// function to
/// retrieve
/// environment
/// variables.
use std::env::var;

/// Importing the "App"
/// structure to create a new
/// Actix Web app.
use actix_web::App;

/// Importing the "Cors"
/// structure to add CORS
/// rules.
use actix_cors::Cors;

/// Importing the "Env"
/// structure to help
/// the Actix Web logger.
use env_logger::Env;

/// Importing this crate's
/// error structure.
use super::err::SentinelErr;

/// Importing the "HttpServer"
/// structure to create an
/// Actix Web app.
use actix_web::HttpServer;

/// Importing the middleware for
/// logging inside Actix Web apps.
use actix_web::middleware::Logger;

/// Importing the "get_system_info"
/// service function to register it.
use super::services::get_system_info;

/// Importing the "check_credentials"
/// service function to register it.
use super::services::check_credentials;

/// Importing the "DefaultHeaders" structure
/// to set custom headers.
use actix_web::middleware::DefaultHeaders;

/// Attempts to run the app.
/// If this fails, an error is returned.
pub async fn run_app() -> Result<(), SentinelErr> {
    let host: String = match var("SENTINEL_HOST"){
        Ok(host) => host,
        Err(e) => return Err::<(), SentinelErr>(SentinelErr::new(&e.to_string()))
    };
    let port: String = match var("SENTINEL_PORT"){
        Ok(host) => host,
        Err(e) => return Err::<(), SentinelErr>(SentinelErr::new(&e.to_string()))
    };
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_addr: String = format!("{}:{}", host, port);
    let server = match HttpServer::new(
        move || {
            let cors = Cors::permissive()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST"]);
            App::new()
                .wrap(cors)
                .wrap(DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", "*"))
                    .add(("Access-Control-Allow-Methods", "GET,POST"))
                    .add(("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept"))
                )
                .wrap(Logger::new("%a %{User-Agent}i"))
                .service(check_credentials)
                .service(get_system_info)
        }
    ).bind(app_addr){
        Ok(server) => server,
        Err(e) => return Err::<(), SentinelErr>(SentinelErr::new(&e.to_string()))
    };
    let running: () = match server.run().await{
        Ok(running) => running,
        Err(e) => return Err::<(), SentinelErr>(SentinelErr::new(&e.to_string()))
    };
    Ok(running)
}