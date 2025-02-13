/*
Sentinel by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting
/// the module for
/// catching and handling
/// errors.
pub use modules::err::*;

/// Re-exporting
/// the module for
/// handling authentication.
pub use modules::auth::*;

/// Re-exporting
/// the module for
/// obtaining system
/// information.
pub use modules::info::*;

/// Re-exporting
/// the module 
/// containing this
/// crate's data structures.
pub use modules::units::*;

/// Re-exporting the
/// module containing
/// the function
/// that runs the
/// application.
pub use modules::runner::*;

/// Re-exporting the module
/// containing this crate's
/// API services.
pub use modules::services::*;