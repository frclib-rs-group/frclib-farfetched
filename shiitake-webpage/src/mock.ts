import * as shiitake from './shiitake_types';

const INTERFERANCE = 0.0;
const TOTAL_RAM = 16 * 1024 * 1024 * 1024;
const CORE_COUNT = 4;

export function mockResources(): Promise<shiitake.Resources | null> {
    return new Promise<shiitake.Resources | null>((resolve, reject) => {
        if (Math.random() < INTERFERANCE) {
            reject("Interferance");
        } else {
            resolve(shiitake.Resources.random(CORE_COUNT, 2, 2, TOTAL_RAM));
        }
    })
    .catch((reason) => {
        console.error(reason);
        return null;
    });
}

export function mockProcesses(): Promise<shiitake.Processes | null> {
    return new Promise<shiitake.Processes | null>((resolve, reject) => {
        if (Math.random() < INTERFERANCE) {
            reject("Interferance");
        } else {
            resolve([
                shiitake.Process.random("robot code", TOTAL_RAM / 3, 0.33),
                shiitake.Process.random("lvrt", TOTAL_RAM / 3, 0.33),
                shiitake.Process.random("frc netcomm", TOTAL_RAM / 3, 0.33),
            ]);
        }
    })
    .catch((reason) => {
        console.error(reason);
        return null;
    }); 
}

export function mockSummary(): Promise<shiitake.Summary> {
    return new Promise<shiitake.Summary>((resolve, reject) => {
        resolve(new shiitake.Summary(
            "mock host",
            "linux gnu mock 2023.3.4",
            "1.1",
            "1.0",
            999,
            CORE_COUNT,
            TOTAL_RAM
        ));
    });
}

export const MOCK_UPTIME = "7b:1c8"
export const MOCK_TIME = "1e240:315"