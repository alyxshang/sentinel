/*
Sentinel by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "App"
/// structure to create a new
/// Actix Web app.
use actix_web::App;

/// Importing the data structure
/// for submitting a payload
/// containing user account
/// credentials.
use super::units::AuthPayload;

/// Importing the "get_sys_info"
/// function to test it.
use super::info::get_sys_info;

/// Importing the "get_system_info"
/// service function to test it.
use super::services::get_system_info;

/// Importing the "check_credentials"
/// service function to test it.
use super::services::check_credentials;

/// Importing the "ContentType" entity
/// to explicitly supply the expected
/// content type to the Actix Web test app.
use actix_web::http::header::ContentType;

/// The function to test the
/// "get_sys_info" function.
#[test]
pub fn test_get_sys_info(){
    match get_sys_info(){
        Ok(info) => {
            assert_eq!((info.os_name.chars().collect::<Vec<char>>().len() > 0), true);
            assert_eq!((info.os_ver.chars().collect::<Vec<char>>().len() > 0), true);
            assert_eq!((info.storage_info.len() > 0), true);
            assert_eq!((info.net_info.netin != "0.000".to_string()), true);
            assert_eq!((info.net_info.netout != "0.000".to_string()), true);
            assert_eq!((info.ram_info.available != "0.00"), true);
            assert_eq!((info.ram_info.taken != "0.00"), true);
            assert_eq!((info.ram_info.free != "0.00"), true);
            assert_eq!((info.uptime.seconds > 0), true);
        },
        Err(e) => eprintln!("{}", e)
    };
}

/// The function to test the
/// "get_system_info" function.
#[actix_web::test]
pub async fn test_get_system_info() {
    use actix_web::test;
    let app = test::init_service(App::new()
        .service(get_system_info))
        .await;
        let req = test::TestRequest::get().uri("/sentinel/info")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
}

/// The function to test the
/// "check_credentials" function.
#[actix_web::test]
pub async fn test_check_credentials() {
    use actix_web::test;
    let app = test::init_service(App::new()
        .service(check_credentials))
        .await;
    let payload: AuthPayload = AuthPayload{ 
        user: "test".to_string(), 
        password: "test".to_string() 
    };
    let req = test::TestRequest::post().uri("/sentinel/auth")
        .insert_header(ContentType::json())
        .set_json(payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}