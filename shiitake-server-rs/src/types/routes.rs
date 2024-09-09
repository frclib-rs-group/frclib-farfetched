
///take in an identifier and a path,
///make them a pub const
macro_rules! route {
    ($identifier:ident, $path:expr) => {
        pub const $identifier: &str = $path;
    };
}

route!(ROOT, "/");
route!(STATS, "/stats");
route!(PROCESSES, "/processes");
route!(TIME, "/time");
route!(REBOOT, "/reboot");
route!(RIO, "/rio");
route!(SYSTEM_SUMMARY, "/system_summary");
route!(UPTIME, "/uptime");
route!(SET_IP, "/set_ip");