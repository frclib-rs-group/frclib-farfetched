import * as shiitake from './shiitake_types';
import * as mock from './mock';

const IS_DEBUG_BUILD = false;

async function shiitakeFetch(route: string, other?: any): Promise<Response> {
    let currentUrl = window.location.pathname;
    if (currentUrl.endsWith("/")) {
        currentUrl = currentUrl.slice(0, -1);
    }
    const url = `${currentUrl}${route}`;
    return fetch(url, other);
}

export async function getProcesses(): Promise<shiitake.Processes | null> {
    if (IS_DEBUG_BUILD) {
        return mock.mockProcesses();
    } else {
        try {
            const response = await shiitakeFetch(shiitake.PROCESSES_ROUTE);
            const json = await response.json();
            console.log(json);
            let processes = [];
            for (let proc of json) {
                processes.push(shiitake.Process.fromJson(proc));
            }
            return processes;
        } catch (reason) {
            console.error("shiitake fetch err" + reason);
            return null;
        }
    }
}

export async function getResources(): Promise<shiitake.Resources | null> {
    if (IS_DEBUG_BUILD) {
        return mock.mockResources();
    } else {
        try {
            const response = await shiitakeFetch(shiitake.RESOURCES_ROUTE);
            const json = await response.json();
            return shiitake.Resources.fromJson(json);
        } catch (reason) {
            console.error("shiitake fetch err" + reason);
            return null;
        }
    }
}

export async function getTime(): Promise<shiitake.TimeSpec> {
    if (IS_DEBUG_BUILD) {
        const text = await new Promise<string>((resolve, reject) => {
            resolve(mock.MOCK_TIME);
        });
        return shiitake.TimeSpec.fromHex(text);
    } else {
        const response = await shiitakeFetch(shiitake.TIME_ROUTE);
        const text_1 = await response.text();
        return shiitake.TimeSpec.fromHex(text_1);
    }
}

export function setTime(time: shiitake.TimeSpec): Promise<Response> {
    if (IS_DEBUG_BUILD) {
        return new Promise((resolve, reject) => {
            resolve(new Response());
        });
    } else {
        return shiitakeFetch(shiitake.TIME_ROUTE, {
            method: "POST",
            body: time.toHex()
        });
    }
}

export async function getUptime(): Promise<shiitake.TimeSpec> {
    if (IS_DEBUG_BUILD) {
        const text = await new Promise<string>((resolve, reject) => {
            resolve(mock.MOCK_UPTIME);
        });
        return shiitake.TimeSpec.fromHex(text);
    } else {
        const response = await shiitakeFetch(shiitake.UPTIME_ROUTE);
        const text_1 = await response.text();
        return shiitake.TimeSpec.fromHex(text_1);
    }
}

export async function getSummary(): Promise<shiitake.Summary> {
    if (IS_DEBUG_BUILD) {
        return mock.mockSummary();
    } else {
        //keep blocking until we get a summary
        let out: shiitake.Summary;
        while (true) {
            try {
                const response = await shiitakeFetch(shiitake.SUMMARY_ROUTE);
                const json = await response.json();
                out = shiitake.Summary.fromJson(json);
                break;
            } catch (reason) {
                console.error("shiitake fetch err" + reason);
            }
        }
        return out;
    }
}
