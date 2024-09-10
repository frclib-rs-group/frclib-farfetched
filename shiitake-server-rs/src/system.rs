use crate::ShiitakeError;
use std::path::PathBuf;

#[cfg(target_vendor = "roborio")]
fn get_uuid() -> Result<u128, ShiitakeError> {
    let path = PathBuf::from("/var/lib/compactrio/atomiczynq.config");
    let file = std::fs::read_to_string(path)?;
    const PREFIX: &str = "Serial=";
    const SERIAL_SIZE: usize = 16;

    let mut serials = Vec::new();

    //pretend this is a regex :3
    let lines = file.lines().collect::<Vec<&str>>();
    for line in lines {
        if line.contains(PREFIX) {
            for i in 0..(line.len() - PREFIX.len()) {
                if line[i..].starts_with(PREFIX) {
                    serials.push(String::from(&line[i + PREFIX.len()..i + PREFIX.len() + SERIAL_SIZE]));
                    break;
                }
            }
        }
    }

    if serials.len() == 0 {
        return Err(ShiitakeError::DataNotFound);
    }

    let mut serial_int = 0u128;
    for serial in serials {
        serial_int += u128::from_str_radix(&serial, 16)?;
    }

    Ok(serial_int)
}

#[cfg(not(target_vendor = "roborio"))]
fn get_uuid() -> Result<u128, ShiitakeError> {
    let path = PathBuf::from("/etc/machine-id");
    let file = std::fs::read_to_string(path)?;
    let stripped = file.trim_end_matches('\n');
    let uuid = u128::from_str_radix(stripped, 16)?;

    Ok(uuid)
}


#[cfg(target_vendor = "roborio")]
fn get_hostname() -> Result<String, ShiitakeError> {
    let path = PathBuf::from("/etc/natinst/share/ni-rt.ini");
    let file = std::fs::read_to_string(path)?;

    for line in file.lines() {
        if line.starts_with("host_name=") {
            return Ok(String::from(&line[10..]).replace("\"", ""));
        }
    }

    Err(ShiitakeError::DataNotFound)
}

#[cfg(not(target_vendor = "roborio"))]
fn get_hostname() -> Result<String, ShiitakeError> {
    let path = PathBuf::from("/etc/hostname");
    let file = std::fs::read_to_string(path)?;
    let stripped = file.trim_end_matches('\n').to_string();

    Ok(stripped)
}

#[cfg(target_vendor = "roborio")]
fn get_os() -> Result<String, ShiitakeError> {
    let path = PathBuf::from("/etc/natinst/share/scs_imagemetadata.ini");
    let file = std::fs::read_to_string(path)?;

    for line in file.lines() {
        let prefix = "IMAGEVERSION = ";
        if line.starts_with(prefix) {
            return Ok(String::from(&line[prefix.len()..]).replace("\"", ""));
        }
    }

    Err(ShiitakeError::DataNotFound)
}

#[cfg(not(target_vendor = "roborio"))]
fn get_os() -> Result<String, ShiitakeError> {
    let path = PathBuf::from("/etc/os-release");
    let file = std::fs::read_to_string(path)?;

    for line in file.lines() {
        if line.starts_with("PRETTY_NAME=") {
            return Ok(String::from(&line[13..]).replace("\"", ""));
        }
    }

    Err(ShiitakeError::DataNotFound)
}


fn cpu_cores() -> Result<u8, ShiitakeError> {
    let path = PathBuf::from("/proc/cpuinfo");
    let file = std::fs::read_to_string(path)?;

    let mut cores = 0;
    for line in file.lines() {
        if line.starts_with("processor") {
            cores += 1;
        }
    }

    Ok(cores)
}

fn total_memory() -> Result<u64, ShiitakeError> {
    let path = PathBuf::from("/proc/meminfo");
    let file = std::fs::read_to_string(path)?;

    for line in file.lines() {
        if line.starts_with("MemTotal:") {
            let mut total = String::new();
            for c in line.chars() {
                if c.is_digit(10) {
                    total.push(c);
                }
            }
            //check the unit
            let lower_line = line.to_lowercase();
            if lower_line.contains("kb") {
                total.push_str("000");
            } else if lower_line.contains("mb") {
                total.push_str("000000");
            } else if lower_line.contains("gb") {
                total.push_str("000000000");
            }
            return Ok(total.parse::<u64>()?);
        }
    }

    Err(ShiitakeError::DataNotFound)
}

pub fn make_summary() -> crate::types::Summary {
    crate::types::Summary {
        hostname: get_hostname().unwrap_or_else(|_| String::from("Unknown")),
        os: get_os().unwrap_or_else(|_| String::from("Unknown")),
        uuid: get_uuid().unwrap_or_else(|_| 0),
        cpu_cores: cpu_cores().unwrap_or_else(|_| 0),
        total_memory: total_memory().unwrap_or_else(|_| 0),
        shiitake_version: env!("CARGO_PKG_VERSION").to_string(),
        webpage_version: env!("SHIITAKE_WEBPAGE_VERSION").to_string(),
    }
}