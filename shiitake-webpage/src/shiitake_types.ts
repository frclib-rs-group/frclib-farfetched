export const RESOURCES_ROUTE = "/stats";
export const PROCESSES_ROUTE = "/processes";
export const TIME_ROUTE = "/time";
export const UPTIME_ROUTE = "/uptime";
export const SUMMARY_ROUTE = "/system_summary";

export class NetworkUsageEntry {
    interface_: string;
    rx: number;
    tx: number;
    constructor(interface_: string, rx: number, tx: number) {
        this.interface_ = interface_;
        this.rx = rx;
        this.tx = tx;
    }

    static fromJson(json: any) {
        let entry = new NetworkUsageEntry(
            json.interface,
            json.rx,
            json.tx
        );
        return entry;
    }
}
export type NetworkUsage = NetworkUsageEntry[];

export class DiskUsageEntry {
    mountPoint: string;
    total: number;
    used: number;
    constructor(mountPoint: string, total: number, used: number) {
        this.mountPoint = mountPoint;
        this.total = total;
        this.used = used;
    }

    static fromJson(json: any) {
        let entry = new DiskUsageEntry(
            json.mount_point,
            json.total,
            json.used
        );
        return entry;
    }
}
export type DiskUsage = DiskUsageEntry[];

export type CpuSpeed = number[];

export type CpuUsage = number[];

export type MemoryUsage = number;

export class Resources {
    cpuSpeed: CpuSpeed;
    cpuUsage: CpuUsage;
    memoryUsage: MemoryUsage;
    networkUsage: NetworkUsage;
    diskUsage: DiskUsage;

    constructor(
        cpuSpeed: CpuSpeed,
        cpuUsage: CpuUsage,
        memoryUsage: MemoryUsage,
        networkUsage: NetworkUsage,
        diskUsage: DiskUsage
        ) {
        this.cpuSpeed = cpuSpeed;
        this.cpuUsage = cpuUsage;
        this.memoryUsage = memoryUsage;
        this.networkUsage = networkUsage;
        this.diskUsage = diskUsage;
    }

    static fromJson(json: any): Resources {
        return new Resources(
            json.cpuSpeed as CpuSpeed,
            json.cpuUsage as CpuUsage,
            json.memoryUsage as MemoryUsage,
            json.networkUsage as NetworkUsage,
            json.diskUsage as DiskUsage
        );
    }

    static random(cores: number, disks: number, networks: number, totalRam: number): Resources {
        let cpuSpeed = [];
        let cpuUsage = [];
        for (let i = 0; i < cores; i++) {
            cpuSpeed.push(Math.random() * 250 + 750);
            cpuUsage.push(Math.random() * 70 + 30);
        }
        let memoryUsage = Math.random() * (totalRam / 2) + (totalRam / 3);
        let networkUsage = [];
        for (let i = 0; i < networks; i++) {
            networkUsage.push(new NetworkUsageEntry(
                `eth${i}`,
                Math.random() * 1000,
                Math.random() * 1000
            ));
        }
        let diskUsage = [];
        for (let i = 0; i < disks; i++) {
            diskUsage.push(new DiskUsageEntry(
                `/dev/sda${i}`,
                Math.random() * 1000000000,
                Math.random() * 1000000000
            ));
        }
        return new Resources(
            cpuSpeed,
            cpuUsage,
            memoryUsage,
            networkUsage,
            diskUsage
        );
    }
}

export class Process {
    pid: number;
    name: string;
    cpuUsage: number;
    memoryUsage: number;

    constructor(pid: number, name: string, cpuUsage: number, memoryUsage: number) {
        this.pid = pid;
        this.name = name;
        this.cpuUsage = cpuUsage;
        this.memoryUsage = memoryUsage;
    }

    static fromJson(json: any): Process {
        return new Process(
            json.pid,
            json.name,
            json.cpuUsage,
            json.memoryUsage
        );
    }

    static random(name: string, totalRam: number, totalCpu: number): Process {
        return new Process(
            Math.floor(Math.random() * 10000),
            name,
            Math.random() * totalCpu,
            Math.random() * totalRam
        );
    }
}

export type Processes = Process[];

export class TimeSpec {
    seconds: number;
    nanoseconds: number;

    constructor(seconds: number, nanoseconds: number) {
        this.seconds = seconds;
        this.nanoseconds = nanoseconds;
    }

    toHex(): string {
        let hexSeconds = this.seconds.toString(16);
        let hexNanoseconds = this.nanoseconds.toString(16);
        return `${hexSeconds}:${hexNanoseconds}`;
    }

    static fromHex(hex: string): TimeSpec {
        let split = hex.split(":");
        let seconds = parseInt(split[0], 16);
        let nanoseconds = parseInt(split[1], 16);
        let timeSpec = new TimeSpec(seconds, nanoseconds);
        return timeSpec;
    }

    toString(): string {
        return `${this.seconds}:${this.nanoseconds}`;
    }
}

export class Summary {
    hostname: string;
    os: string;
    shiitakeVersion: string;
    webpageVersion: string;
    uuid: number;
    cpuCores: number;
    totalMemory: number;

    constructor(
        hostname: string,
        os: string,
        shiitakeVersion: string,
        webpageVersion: string,
        uuid: number,
        cpuCores: number,
        totalMemory: number
    ) {
        this.hostname = hostname;
        this.os = os;
        this.shiitakeVersion = shiitakeVersion;
        this.webpageVersion = webpageVersion;
        this.uuid = uuid;
        this.cpuCores = cpuCores;
        this.totalMemory = totalMemory;
    }

    static fromJson(json: any): Summary {
        return new Summary(
            json.hostname,
            json.os,
            json.shiitakeVersion,
            json.webpageVersion,
            json.uuid,
            json.cpuCores,
            json.totalMemory
        );
    }
}
