#![allow(dead_code)]

use std::{path::PathBuf, io::Write};
use axum::routing::{get, post};

use axum::http::StatusCode;

macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        tracing::error!($($arg)*);
    }
}

fn get_serial(path: PathBuf) -> Result<String, RioDataError> {
    let file = std::fs::read_to_string(path)?;
    const PREFIX: &str = "Serial=";
    const SERIAL_SIZE: usize = 16;

    let mut serial = "0000000000000000";

    //pretend this is a regex :3
    let lines = file.lines().collect::<Vec<&str>>();
    for line in lines {
        if line.contains(PREFIX) {
            for i in 0..(line.len() - PREFIX.len()) {
                if line[i..].starts_with(PREFIX) {
                    serial = &line[i + PREFIX.len()..i + PREFIX.len() + SERIAL_SIZE];
                    break;
                }
            }
        }
    }

    let mut serial = serial.to_string();
    while serial.starts_with('0') {
        serial = serial[1..].to_string();
    }

    Ok(serial)
}

fn fix_comment_hex(hex_str: String) -> String {
    let mut new_str = String::new();
    for char in hex_str.chars() {
        let orded = char as u8;
        if 58 <= orded && orded <= 63 {
            new_str.push((orded + 39) as char);
        } else {
            new_str.push(char);
        }
    }
    new_str
}
fn unfix_comment_hex(hex_str: String) -> String {
    let mut new_str = String::new();
    for char in hex_str.chars() {
        let orded = char as u8;
        if 97 <= orded && orded <= 102 {
            new_str.push((orded - 39) as char);
        } else {
            new_str.push(char);
        }
    }
    new_str
}
fn swap_hex(hex_str: String) -> String {
    format!("{}{}", &hex_str[1..], &hex_str[..1])
}
fn comment_hex_to_int(hex_str: String) -> i64 {
    i64::from_str_radix(&swap_hex(fix_comment_hex(hex_str)), 16)
        .expect("Failed to convert hex to int")
}

fn decode_comment(encoded_str: String) -> String {
    let mut decoded_str = String::new();
    for i in (0..encoded_str.len()).step_by(2) {
        let hex_str = &encoded_str[i..i+2];
        let int_val = comment_hex_to_int(hex_str.to_string());
        let ascii_val = int_val as u8 as char;
        decoded_str.push(ascii_val);
    }
    decoded_str
}

fn encode_comment(decoded_str: String) -> String {
    let mut encoded_str = String::new();
    for char in decoded_str.chars() {
        let int_val = char as u8;
        let hex_str = format!("{:x}", int_val);
        encoded_str.push_str(&unfix_comment_hex(swap_hex(hex_str)));
    }
    encoded_str
}

fn set_hostname(hostname: String) -> Result<(), RioDataError> {
    let mut file = std::fs::File::create(PathBuf::from("/etc/hostname"))?;
    file.write_all(hostname.as_bytes())?;
    Ok(())
}

use axum::Router;
use serde_json::{Map, Value};
use static_init::dynamic;
use crate::ini::{read_ini, read_ini_field, Ini, IniSection, IniTypes};
use crate::routes;

type RioDataError = crate::ini::IniError;

#[derive(Debug)]
struct RioData {
    serial: String,
    rt_ini: Ini,
    image_version: String,
}

impl RioData {
    fn new() -> Result<Self, RioDataError>{
        Ok(RioData {
            serial: get_serial(PathBuf::from("/var/lib/compactrio/atomiczynq.config"))?,
            rt_ini: read_ini(PathBuf::from("/etc/natinst/share/ni-rt.ini"))?,
            image_version: read_ini_field(
                PathBuf::from("/etc/natinst/share/scs_imagemetadata.ini"),
                "ImageMetadata",
                "IMAGEVERSION",
            )?.to_string()
            .ok_or(RioDataError::CustomError("Failed to convert image version to string"))?,
        })
    }

    fn to_json_map(&self) -> Result<Map<String, Value>, RioDataError> {
        let mut map = Map::new();
        map.insert("serial".into(), Value::String(self.serial.clone()));
        map.insert("image_version".into(), Value::String(self.image_version.clone()));

        let system_settings = self.rt_ini
            .get("systemsettings")
            .ok_or(RioDataError::SectionDoesntExist)?;

        map.insert("no_fpga_app".into(), Value::Bool(system_settings
            .get("NoFPGAApp.enabled")
            .ok_or(RioDataError::KeyDoesntExist)?
            .clone().to_boolean_from_string()
            .ok_or(RioDataError::CustomError("Failed to convert NoFPGAApp.enabled to string"))?)
        );
        map.insert("console_out".into(), Value::Bool(system_settings
            .get("ConsoleOut.enabled")
            .ok_or(RioDataError::KeyDoesntExist)?
            .clone().to_boolean_from_string()
            .ok_or(RioDataError::CustomError("Failed to convert ConsoleOut.enabled to string"))?)
        );
        map.insert("no_app".into(), Value::Bool(system_settings
            .get("NoApp.enabled")
            .ok_or(RioDataError::KeyDoesntExist)?
            .clone().to_boolean_from_string()
            .ok_or(RioDataError::CustomError("Failed to convert NoApp.enabled to string"))?)
        );
        map.insert("safe_mode".into(), Value::Bool(system_settings
            .get("SafeMode.enabled")
            .ok_or(RioDataError::KeyDoesntExist)?
            .clone().to_boolean_from_string()
            .ok_or(RioDataError::CustomError("Failed to convert SafeMode.enabled to string"))?)
        );
        map.insert("host_name".into(), Value::String(system_settings
            .get("host_name")
            .ok_or(RioDataError::KeyDoesntExist)?
            .clone().to_string()
            .ok_or(RioDataError::CustomError("Failed to convert host_name to string"))?)
        );
        map.insert("comment".into(), Value::String(decode_comment(system_settings
            .get("Comment")
            .ok_or(RioDataError::KeyDoesntExist)?
            .clone().to_string()
            .ok_or(RioDataError::CustomError("Failed to convert Comment to string"))?))
        );

        Ok(map)
    }
}

#[dynamic(lazy)]
static mut RIO_DATA: RioData = match RioData::new() {
    Ok(rio_data) => rio_data,
    Err(_e) => {
        error!("Failed to get rio data: {:?}", _e);
        panic!("Failed to get rio data");
    }
};

pub fn init_rio(app: Router) -> Router {
    app
    .route(routes::RIO, get(get_rio))
    .route(routes::RIO, post(set_rio))
    .route("/nisysdetails/ping", get(|| async { "SHIITAKE" }))
}

async fn get_rio() -> Result<String, StatusCode> {
    match RIO_DATA.read().to_json_map() {
        Ok(map) => Ok(serde_json::to_string(&map).unwrap()),
        Err(_e) => {
            error!("Failed to get rio data: {:?}", _e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn boolc(b: bool) -> String {
    if b {
        "True".to_string()
    } else {
        "False".to_string()
    }
}

async fn set_rio(map_str: String) -> Result<(), StatusCode> {
    let map: Map<String, Value> = serde_json::from_str(&map_str)
        .map_err(|_| {error!("Failed to parse json"); StatusCode::BAD_REQUEST})?;
    let mut rio_data = RIO_DATA.write();
    let system_settings = rio_data.rt_ini
        .get_mut("systemsettings")
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    for (key, value) in map {
        match key.as_str() {
            "no_fpga_app" => system_settings.set(
                "NoFPGAApp.enabled",
                IniTypes::Boolean(value.as_bool().ok_or(StatusCode::BAD_REQUEST)?)
            ).map_err(|_| {error!("NoFPGAApp.enabled set err"); StatusCode::INTERNAL_SERVER_ERROR})?,
            "console_out" => system_settings.set(
                "ConsoleOut.enabled",
                IniTypes::Boolean(value.as_bool().ok_or(StatusCode::BAD_REQUEST)?)
            ).map_err(|_| {error!("ConsoleOut.enabled set err"); StatusCode::INTERNAL_SERVER_ERROR})?,
            "no_app" => system_settings.set(
                "NoApp.enabled",
                IniTypes::String(boolc(value.as_bool().ok_or(StatusCode::BAD_REQUEST)?))
            ).map_err(|_| {error!("NoApp.enabled set err"); StatusCode::INTERNAL_SERVER_ERROR})?,
            "safe_mode" => system_settings.set(
                "SafeMode.enabled", 
                IniTypes::String(boolc(value.as_bool().ok_or(StatusCode::BAD_REQUEST)?))
            ).map_err(|_| {error!("SafeMode.enabled set err"); StatusCode::INTERNAL_SERVER_ERROR})?,
            "host_name" => {
                let name = value.as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string();
                set_hostname(name.clone()).map_err(|_| {error!("host_name sfile write err"); StatusCode::INTERNAL_SERVER_ERROR})?;
                system_settings.set(
                    "host_name",
                IniTypes::String(name)
                ).map_err(|_| {error!("host_name set err"); StatusCode::INTERNAL_SERVER_ERROR})?
            },
            "comment" => system_settings.set(
                "Comment",
                IniTypes::String(encode_comment(value.as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string()))
            ).map_err(|_| {error!("Comment set err"); StatusCode::INTERNAL_SERVER_ERROR})?,
            _ => return Err(StatusCode::BAD_REQUEST)
        }
    }
    rio_data.rt_ini.save()
        .map_err(|_| {error!("ni-rt.ini save err"); StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(())
}

pub fn write_static_ip(ip: String, gateway: String, dns: String) {
    let ini = &mut RIO_DATA.write().rt_ini;
    let mut section = IniSection::new("eth0");
    section.create_and_set("dhcpenabled", IniTypes::String("0".to_string()));
    section.create_and_set("linklocalenabled", IniTypes::String("0".to_string()));
    section.create_and_set("IP_Address", IniTypes::String(ip));
    section.create_and_set("Subnet_Mask", IniTypes::String("255.255.255.0".to_string()));
    section.create_and_set("Gateway", IniTypes::String(gateway));
    section.create_and_set("DNS_Address", IniTypes::String(dns));
    section.create_and_set("Mode", IniTypes::String("TCPIP".to_string()));
    section.create_and_set("MediaMode", IniTypes::String("Auto".to_string()));
    ini.create_and_set(section);
    let _ = ini.save();
}

pub fn write_dhcp(ip: String) {
    let ini = &mut RIO_DATA.write().rt_ini;
    let mut section = IniSection::new("eth0");
    section.create_and_set("dhcpenabled", IniTypes::String("1".to_string()));
    section.create_and_set("linklocalenabled", IniTypes::String("1".to_string()));
    section.create_and_set("Mode", IniTypes::String("TCPIP".to_string()));
    section.create_and_set("MediaMode", IniTypes::String("Auto".to_string()));
    section.create_and_set("dhcpipaddr", IniTypes::String(ip));
    ini.create_and_set(section);
    let _ = ini.save();
}




#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_read_ini() {
        let ini = read_ini(PathBuf::from("test.ini")).unwrap();
        assert_eq!(ini.get("Default").unwrap().get("Port").unwrap(), &IniTypes::Integer(80));
        assert_eq!(ini.get("Default").unwrap().get("SSLEnabled").unwrap(), &IniTypes::Boolean(false));
        assert_eq!(ini.get("Hosts").unwrap().get("Default").unwrap(), &IniTypes::String("Default Host".to_string()));
    }

    #[test]
    fn test_get_serial() {
        assert_eq!(get_serial(PathBuf::from("serial.test")).unwrap(), "306ADDC".to_string());
    }
}