/*
Sentinel by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Disks"
/// structure to get info
/// on available disks.
use sysinfo::Disks;

/// Importing the
/// "System" structure
/// to get system info.
use sysinfo::System;

/// Importing the "Networks"
/// structure to get info
/// on available networks.
use sysinfo::Networks;

/// Importing the "Platform"
/// trait to get the uptime.
use systemstat::Platform;

/// Imnporting the structure
/// to store info on uptime.
use super::units::UpTime;

/// Importing the data structure
/// for storing system information.
use super::units::SysInfo;

/// Importing the structure
/// to save information on
/// system memory.
use super::units::RAMInfo;

/// Importing this crate's error-handling
/// structure to catch and handle errors.
use super::err::SentinelErr;

/// Importing the structure
/// to save information on
/// network activity.
use super::units::TrafficInfo;

/// Importing the structure
/// to save information on
/// system storage.
use super::units::DiskInfo;

/// Importing a different "System"
/// structure to get info on uptime.
use systemstat::System as UptimeStat;

/// This function attempts to get the uptime
/// for the current system.
pub fn get_uptime() -> Result<UpTime, SentinelErr>{
    let stat = UptimeStat::new();
    let raw_uptime: u64 = match stat.uptime(){
        Ok(uptime) => uptime.as_secs(),
        Err(e) => return Err::<UpTime, SentinelErr>(SentinelErr::new(&e.to_string()))
    };
    let hours: u64 = raw_uptime / 3600;
    let minutes: u64 = (raw_uptime - (hours * 3600)) / 60;
    let hours_and_minutes: u64 = (hours * 3600) + (minutes * 60);
    let seconds: u64 = raw_uptime - hours_and_minutes;
    Ok(
        UpTime{
            hours: hours,
            minutes: minutes,
            seconds: seconds,
        }
    )
}

/// This function retrieves information
/// on network activity and returns it
/// in the "TrafficInfo" structure.
pub fn get_net_stats() -> TrafficInfo {
    let mut netin: f64 = 0f64;
    let mut netout: f64 = 0f64;
    let divisor: f64 = 1024f64 * 1024f64 * 1024f64;
    let networks = Networks::new_with_refreshed_list();
    for (_interface, data) in &networks {
        let recv: f64 = data.total_received() as f64;
        let out: f64 = data.total_transmitted() as f64;
        netin = netin + recv;
        netout = netout + out;
    }
    netin = netin / divisor;
    netout = netout / divisor;
    TrafficInfo{ netin: format!("{:.3}",netin), netout: format!("{:.3}",netout) }
}

/// This function retrieves information
/// on storage disks and returns it
/// in the "StorageInfo" structure.
pub fn get_storage_info() -> Result<Vec<DiskInfo>,SentinelErr> {
    let mut result: Vec<DiskInfo> = Vec::new();
    let divisor: f64 = 1024f64 * 1024f64 * 1024f64;
    for disk in &Disks::new_with_refreshed_list(){
        let total_space: f64 = disk.total_space() as f64 / divisor ;
        let free_space: f64 = disk.available_space() as f64 / divisor;
        let taken_space: f64 = total_space - free_space;
        let disk_name: String = match disk.name().to_str(){
            Some(disk_name) => disk_name.to_string(),
            None => return Err::<Vec<DiskInfo>,SentinelErr>(SentinelErr::new(&"Could not get disk name.".to_string()))
        };
        result.push(
            DiskInfo{
                name: disk_name,
                available: format!("{:.2}", total_space),
                taken: format!("{:.2}", taken_space),
                free: format!("{:.2}", free_space)
            }
        );
    }
    Ok(result)
}

/// This function retrieves information
/// on system memory and returns it
/// in the "StorageInfo" structure.
pub fn get_ram_info(subject: &mut System) -> RAMInfo{
    let available: f64 = (subject.total_memory() as f64 + subject.total_swap() as f64)/ 1000000000f64;
    let taken: f64 = (subject.used_memory()as f64 + subject.free_swap() as f64) / 1000000000f64;
    RAMInfo{ 
        available: format!("{:.2}",available),
        taken: format!("{:.2}", taken),
        free: format!("{:.2}",available - taken)
    }
}

/// This function attempts to retrieve all information
/// stored in the "SysInfo" structure and return it.
/// If this cannot be done, an error is returned.
pub fn get_sys_info() -> Result<SysInfo, SentinelErr>{
    let mut my_system = System::new_all();
    my_system.refresh_all();
    let uptime: UpTime = match get_uptime(){
        Ok(uptime) => uptime,
        Err(_e) => return Err::<SysInfo, SentinelErr>(SentinelErr::new(&"Could not get uptime info.".to_string()))
    };
    let storage: Vec<DiskInfo> = match get_storage_info(){
        Ok(storage) => storage,
        Err(_e) => return Err::<SysInfo, SentinelErr>(SentinelErr::new(&"Could not get storage info.".to_string()))
    };
    let traffic_info = get_net_stats();
    let ram_info: RAMInfo = get_ram_info(&mut my_system);
    let sname: String = match System::name(){
        Some(sname) => sname,
        None => return Err::<SysInfo, SentinelErr>(SentinelErr::new(&"Could not get system name.".to_string()))
    };
    let sver: String = match System::os_version(){
        Some(sver) => sver,
        None => return Err::<SysInfo, SentinelErr>(SentinelErr::new(&"Could not get system version.".to_string()))
    };
    Ok(
        SysInfo {
            uptime: uptime,
            storage_info: storage,
            ram_info: ram_info,
            net_info: traffic_info,
            os_name: sname,
            os_ver: sver
        }
    )
}