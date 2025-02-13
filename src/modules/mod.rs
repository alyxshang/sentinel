/*
Sentinel by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting
/// the module for
/// catching and handling
/// errors.
pub mod err;

/// Exporting
/// the module for
/// obtaining system
/// information.
pub mod info;

/// Exporting the module
/// that contains this
/// crate's tests.
#[cfg(test)]
pub mod tests;

/// Exporting
/// the module for
/// handling authentication.
pub mod auth;

/// Exporting
/// the module 
/// containing this
/// crate's data structures.
pub mod units;

/// Exporting the
/// module containing
/// the function
/// that runs the
/// application.
pub mod runner;

/// Exporting the module
/// containing this crate's
/// API services.
pub mod services;