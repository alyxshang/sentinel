/*
Sentinel by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "get"
/// decorator to make a service
/// that accepts "GET" requests.
use actix_web::get;

/// Importing the "post"
/// decorator to make a service
/// that accepts "POST" requests.
use actix_web::post;

/// Importing the "Result"
/// enum for Actix Web services.
use actix_web::Result;

/// Importing the "Json"
/// structure to return JSON
/// responses.
use actix_web::web::Json;

/// Importing the data structure
/// for storing system information.
use super::units::SysInfo;

/// Importing the function
/// to return a HTTP response.
use actix_web::HttpResponse;

/// Importing this crate's error-handling
/// structure to catch and handle errors.
use super::err::SentinelErr;

/// Importing the structure
/// containing the payload with
/// user account credentials.
use super::units::AuthPayload;

/// Importing the function to
/// attempt to retrieve system
/// information.
use super::info::get_sys_info;

/// Importing the "AuthResponse"
/// structure to return info
/// on whether authentication
/// was successful or not.
use super::units::AuthResponse;

/// Importing the function to
/// attempt to check user credentials.
use super::auth::authorize_user;

/// This function is an
/// Actix Web service that
/// receives user crednetials
/// supplied through the
/// "AuthPayload" structure
/// and returns a boolean response
/// based on whether the
/// credentials are valid or not.
#[post("/sentinel/auth")]
pub async fn check_credentials(
    payload: Json<AuthPayload>,
) -> Result<HttpResponse, SentinelErr> {
    let resp: AuthResponse = match authorize_user(
        &payload.user,
        &payload.password 
    ){
        Ok(resp) => resp,
        Err(e) => return Err::<HttpResponse, SentinelErr>(SentinelErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(resp))
}

/// This function is an
/// Actix Web service that
/// returns system information
/// as a JSON object in the shape
/// of the "SysInfo" structure.
#[get("/sentinel/info")]
pub async fn get_system_info() -> Result<HttpResponse, SentinelErr>{
    let resp: SysInfo = match get_sys_info(){
        Ok(resp) => resp,
        Err(e) => return Err::<HttpResponse, SentinelErr>(SentinelErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(resp))
}