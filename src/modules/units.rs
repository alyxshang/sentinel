/*
Sentinel by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// macro for
/// serializing Rust data
/// structures into JSON.
use serde::Serialize;

/// Importing the
/// macro for
/// deserializing Rust data
/// structures into JSON.
use serde::Deserialize;

/// Declaring the data structure
/// for obtaining and returning
/// information on the system.
#[derive(Serialize)]
pub struct SysInfo {
    pub uptime: UpTime,
    pub storage_info: Vec<DiskInfo>,
    pub ram_info: RAMInfo,
    pub net_info: TrafficInfo,
    pub os_name: String,
    pub os_ver: String,
}

/// Declaring the data structure
/// for obtaining and checking
/// user information.
#[derive(Serialize,Deserialize)]
pub struct AuthPayload{
    pub user: String,
    pub password: String
}

/// Declaring the data structure
/// for returning info on
/// whether user authentication
/// was successful or not.
#[derive(Serialize)]
pub struct AuthResponse {
    pub status: bool
}

/// A structure to save
/// info on a single disk.
#[derive(Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub available: String,
    pub taken: String,
    pub free: String
}

/// A structure to save
/// info on system memory.
#[derive(Serialize)]
pub struct RAMInfo {
    pub available: String,
    pub taken: String,
    pub free: String
}

/// A structure to save
/// info on network traffic.
#[derive(Serialize)]
pub struct TrafficInfo{
    pub netin: String,
    pub netout: String
}

/// A structure to save
/// info on system uptime.
#[derive(Serialize)]
pub struct UpTime{
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64
}