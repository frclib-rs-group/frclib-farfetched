

use crate::types::{DiskUsageEntry, NetworkUsageEntry, Process, Processes, Stats};

use sysinfo::{CpuExt, DiskExt, NetworkExt, PidExt, ProcessExt, System, SystemExt};

pub fn measure_stats(system: &mut System) -> Stats {
    let mut stats: Stats = Default::default();

    system.refresh_cpu();
    system.refresh_memory();
    system.refresh_networks();
    system.refresh_disks();

    let cpus = system.cpus();

    stats.cpu_speed = Some(cpus.iter().map(|cpu| cpu.frequency() * 1_000_000).collect::<Vec<_>>());
    stats.cpu_usage = Some(cpus.iter().map(|cpu| cpu.cpu_usage() as f64).collect::<Vec<_>>());
    stats.memory_usage = Some(system.used_memory());

    stats.network_usage = Some(
        system
            .networks()
            .into_iter()
            .map(|(name, data)| NetworkUsageEntry {
                interface: name.clone(),
                rx: data.received(),
                tx: data.transmitted(),
            })
            .collect::<Vec<_>>(),
    );

    stats.disk_usage = Some(
        system
            .disks()
            .into_iter()
            .map(|disk| DiskUsageEntry {
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total: disk.total_space(),
                used: disk.total_space() - disk.available_space(),
            })
            .collect::<Vec<_>>(),
    );
    stats
}

pub fn measure_processes(system: &mut System) -> Processes {
    let mut processes: Processes = Default::default();

    system.refresh_processes();

    let cpu_count = system.cpus().len() as f64;

    for (sys_pid, sys_process) in system.processes() {
        let cpu_usage = sys_process.cpu_usage();
        let memory_usage = sys_process.memory();
        if memory_usage == 0 && cpu_usage == 0.0 {
            continue;
        }

        let process = Process {
            name: sys_process.exe().file_name().unwrap().to_string_lossy().to_string(),
            cpu_usage: cpu_usage as f64 / cpu_count,
            memory_usage,
            pid: sys_pid.as_u32(),
        };

        processes.push(process);
    }

    processes
}
