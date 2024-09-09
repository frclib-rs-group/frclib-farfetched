
use serde::{Deserialize, Serialize};


pub type Processes = Vec<Process>;
pub mod routes;

pub const REBOOT_VERIFICATION: &str = "please";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkUsageEntry {
    pub interface: String,
    pub rx: u64,
    pub tx: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiskUsageEntry {
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_speed: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_usage: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_usage: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_usage: Option<Vec<NetworkUsageEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_usage: Option<Vec<DiskUsageEntry>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub hostname: String,
    pub os: String,
    pub shiitake_version: String,
    pub webpage_version: String,
    pub uuid: u128,
    pub cpu_cores: u8,
    pub total_memory: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f64,
    pub memory_usage: u64,
}

pub fn timespec_to_hex(seconds: i64, nanoseconds: i64) -> String {
    let hex_seconds = format!("{:x}", seconds);
    let hex_nanoseconds = format!("{:x}", nanoseconds);
    format!("{}:{}", hex_seconds, hex_nanoseconds)
}

pub fn hex_to_timespec(hex: String) -> (i64, i64) {
    let split = hex.split(":");
    let mut split = split.into_iter();
    let seconds = i64::from_str_radix(split.next().unwrap(), 16).unwrap();
    let nanoseconds = i64::from_str_radix(split.next().unwrap(), 16).unwrap();
    (seconds, nanoseconds)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_hex_int() {
        let timespec_mock = (123456, 789);
        let hex = super::timespec_to_hex(timespec_mock.0, timespec_mock.1);
        println!("{}", hex);
        let timespeck_modck2 = super::hex_to_timespec(hex);
        assert_eq!(timespec_mock.0, timespeck_modck2.0);
        assert_eq!(timespec_mock.1, timespeck_modck2.1);
    }

    #[test]
    fn test_stat() {
        use super::Stats;
        let stats_mock = Stats {
            cpu_speed: Some(vec![1000, 2000, 3000]),
            cpu_usage: Some(vec![1.0, 2.0, 3.0]),
            memory_usage: Some(20000),
            network_usage: Some(vec![
                super::NetworkUsageEntry {
                    interface: "eth0".to_string(),
                    rx: 1000,
                    tx: 2000,
                },
                super::NetworkUsageEntry {
                    interface: "eth1".to_string(),
                    rx: 3000,
                    tx: 4000,
                },
            ]),
            disk_usage: Some(vec![
                super::DiskUsageEntry {
                    mount_point: "/".to_string(),
                    total: 100000,
                    used: 20000,
                },
                super::DiskUsageEntry {
                    mount_point: "/home".to_string(),
                    total: 300000,
                    used: 40000,
                },
            ]),
        };
        let stats_mock_json = serde_json::to_string(&stats_mock).unwrap();
        println!("{}", stats_mock_json);
    }

    #[test]
    fn test_proccesses() {
        let processes_mock = vec![
            super::Process {
                pid: 1,
                name: "init".to_string(),
                cpu_usage: 0.1,
                memory_usage: 1000,
            },
            super::Process {
                pid: 2,
                name: "systemd".to_string(),
                cpu_usage: 0.2,
                memory_usage: 2000,
            },
            super::Process {
                pid: 3,
                name: "robo".to_string(),
                cpu_usage: 82.0,
                memory_usage: 90000,
            },
        ];
        let processes_mock_json = serde_json::to_string(&processes_mock).unwrap();
        println!("{}", processes_mock_json);
    }
}
