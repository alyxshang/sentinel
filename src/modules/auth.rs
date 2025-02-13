/*
Sentinel by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "pwcheck"
/// function to check user
/// credentials.
use pwcheck::pwcheck;

/// Importiing the enum to
/// catch different types of
/// password-verification
/// failures.
use pwcheck::PwcheckResult;

/// Importing this crate's error-handling
/// structure to catch and handle errors.
use super::err::SentinelErr;

/// Importing the "AuthResponse"
/// structure to return info
/// on whether authentication
/// was successful or not.
use super::units::AuthResponse;

/// This function takes the
/// username and password of
/// a certain user account
/// and checks if these credentials
/// match any of the users present
/// on the system.
pub fn authorize_user(
    user: &str,
    password: &str
) -> Result<AuthResponse, SentinelErr>{
    let status: bool = match pwcheck(user, password){
        PwcheckResult::Ok => true,
        PwcheckResult::WrongPassword => false,
        PwcheckResult::Err(e) => return Err::<AuthResponse, SentinelErr>(SentinelErr::new(&e.to_string()))
    };
    Ok( AuthResponse { status: status })

}