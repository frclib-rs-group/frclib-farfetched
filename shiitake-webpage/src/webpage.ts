import * as shiitake from './shiitake_types';
import * as fetcher from './fetcher';
import { putGraph, putGraphSingle } from './grapher';

function average(array: number[]) {
    return array.reduce((a, b) => a + b) / array.length;
}

function sum(array: number[]) {
    return array.reduce((a, b) => a + b);
}

function formatHertz(hertz: number): string {
    if (hertz < 1000) {
        return `${(hertz).toFixed(2)} Hz`;
    } else if (hertz < 1000000) {
        return `${(hertz / 1000).toFixed(2)} KHz`;
    } else if (hertz < 1000000000) {
        return `${(hertz / 1000000).toFixed(2)} MHz`;
    } else {
        return `${(hertz / 1000000000).toFixed(2)} GHz`;
    }
}

function formatBytes(bytes: number): string {
    if (bytes < 1000) {
        return `${(bytes).toFixed(2)} B`;
    } else if (bytes < 1000000) {
        return `${(bytes / 1000).toFixed(2)} KB`;
    } else if (bytes < 1000000000) {
        return `${(bytes / 1000000).toFixed(2)} MB`;
    } else {
        return `${(bytes / 1000000000).toFixed(2)} GB`;
    }
}

function formatBytesPerSec(bytes: number): string {
    return formatBytes(bytes) + "/s";
}

function formatPercent(percent: number): string {
    return `${percent.toFixed(2)}%`;
}

let SUMMARY = await fetcher.getSummary();

class ResourceManager {
    resources: (shiitake.Resources | null)[] = [];
    windowSize: number
    constructor(windowSize: number) {
        this.windowSize = windowSize;
    }

    public getLatest(): shiitake.Resources | null {
        if (this.resources.length == 0) {
            return null;
        }
        return this.resources[this.resources.length - 1];
    }

    add(resources: shiitake.Resources) {
        this.resources.push(resources);
        if (this.resources.length > this.windowSize) {
            this.resources.shift();
        }
    }

    addEmpyty() {
        this.resources.push(null);
        if (this.resources.length > this.windowSize) {
            this.resources.shift();
        }
    }

    update() {
        if (this.resources.length == 0) {
            return;
        }

        let resourcesElement = document.getElementById("sys-rsrcs");
        if (!resourcesElement) {
            console.error("Could not find sys-rsrcs element");
            return;
        }

        //find a resource child with id "cpu-usage"
        let cpuUsage = resourcesElement.querySelector("#cpu-usage")!;
        this.updateCpuUsage(cpuUsage as HTMLElement);

        //find a resource child with id "cpu-speed"
        let cpuSpeed = resourcesElement.querySelector("#cpu-speed")!;
        this.updateCpuSpeed(cpuSpeed as HTMLElement);

        //find a resource child with id "memory-usage"
        let memoryUsage = resourcesElement.querySelector("#memory-usage")!;
        this.updateMemoryUsage(memoryUsage as HTMLElement);
    }

    updateCpuUsage(elem: HTMLElement) {
        let details = elem as HTMLDetailsElement;
        let text = details.children[0].children[0] as HTMLSpanElement;
        let usageBar = details.children[0].children[1] as HTMLProgressElement;
        let graphDiv = details.children[1] as HTMLDivElement;

        let latestResources = this.resources[this.resources.length - 1];
        if (latestResources) {
            let val = average(latestResources.cpuUsage);
            text.innerText = "CPU Usage: " + formatPercent(val);
            usageBar.value = val;
            //make a color gradient from green to red
            let red = Math.floor(200 * (val / 100));
            let green = 255 - red;
            usageBar.style.setProperty("--progress-color", "rgb(" + red + "," + green + ",0)");
        } else {
            text.innerText = "CPU Usage: N/A";
            usageBar.value = 0;
        }

        if (details.open) {
            let data = this.resources.map(r => r ? r.cpuUsage as number[] : null);
            putGraph(graphDiv, data, (i) => "CPU" + i, this.windowSize, [0, 100], false);
        }
    }

    updateCpuSpeed(elem: HTMLElement) {
        let text = elem as HTMLSpanElement;
        let latestResources = this.resources[this.resources.length - 1];
        if (latestResources) {
            text.innerText = "CPU Speed: " + formatHertz(average(latestResources.cpuSpeed));
        } else {
            text.innerText = "CPU Speed: N/A";
        }
    }

    updateMemoryUsage(elem: HTMLElement) {
        let details = elem as HTMLDetailsElement;
        let text = details.children[0].children[0] as HTMLSpanElement;
        let usageBar = details.children[0].children[1] as HTMLProgressElement;
        let graphDiv = details.children[1] as HTMLDivElement;

        let latestResources = this.resources[this.resources.length - 1];
        if (latestResources) {
            let val = latestResources.memoryUsage;
            text.innerText = "Memory Usage: " + formatBytes(val);
            usageBar.value = val;
            usageBar.max = SUMMARY.totalMemory;
            //make a color gradient from green to red
            let red = Math.floor(200 * (val / SUMMARY.totalMemory));
            let green = 255 - red;
            usageBar.style.setProperty("--progress-color", "rgb(" + red + "," + green + ",0)");
        } else {
            text.innerText = "Memory Usage: N/A";
            usageBar.value = 0;
        }

        if (details.open) {
            let data = this.resources.map(r => r ? r.memoryUsage as number : null);
            putGraphSingle(graphDiv, data, (_) => "Memory", this.windowSize, [0, SUMMARY.totalMemory], "#4f00ff");
        }
    }


}

export async function updateUptime() {
    let uptime = await fetcher.getUptime().then(uptime => uptime.toString());
    let uptimeElement = document.getElementById("uptime");
    if (uptimeElement) {
        uptimeElement.innerText = `Uptime: ${uptime.split(":")[0]}s`;
    }
}

const resourceManager = new ResourceManager(window.innerWidth < 600 ? 30 : 60);

export async function updateResources() {
    let resources = await fetcher.getResources();
    if (resources == null) {
        resourceManager.addEmpyty();
    } else {
        resourceManager.add(resources);
    }
    resourceManager.update();
}

let procOrderType: "cpu" | "name" | "mem" | "pid" = "cpu";
let orderAscending = false;

export async function updateProcesses() {
    let processes = await fetcher.getProcesses();
    let tableData = document.getElementById("processes-table-data");

    if (tableData == null) {
        console.error("Could not find processes-table-data element");
        return;
    }

    if (processes == null) {
        //clear the table
        tableData.innerHTML = "";
        return;
    }

    if (procOrderType == "cpu") {
        if (orderAscending) {
            processes.sort((a, b) => a.cpuUsage - b.cpuUsage);
        } else {
            processes.sort((a, b) => b.cpuUsage - a.cpuUsage);
        }
    } else if (procOrderType == "name") {
        if (orderAscending) {
            processes.sort((a, b) => a.name.localeCompare(b.name));
        } else {
            processes.sort((a, b) => b.name.localeCompare(a.name));
        }
    } else if (procOrderType == "mem") {
        if (orderAscending) {
            processes.sort((a, b) => a.memoryUsage - b.memoryUsage);
        } else {
            processes.sort((a, b) => b.memoryUsage - a.memoryUsage);
        }
    } else if (procOrderType == "pid") {
        if (orderAscending) {
            processes.sort((a, b) => a.pid - b.pid);
        } else {
            processes.sort((a, b) => b.pid - a.pid);
        }
    }

    let tableHtml = "";
    let accumulatedCpuUsage = 0;
    let accumulatedMemoryUsage = 0;
    let counter = 0;
    for (let proc of processes) {
        accumulatedCpuUsage += proc.cpuUsage;
        accumulatedMemoryUsage += proc.memoryUsage;
        tableHtml += `
            <tr>
                <td>${proc.pid}</td>
                <td>${proc.name}</td>
                <td>${formatPercent(proc.cpuUsage)}</td>
                <td>${formatBytes(proc.memoryUsage)}</td>
            </tr>
        `;
        counter++;
        if (counter >= 10) {
            break;
        }
    }

    let rsrcs = resourceManager.getLatest();
    if (rsrcs) {
        let otherCpuUsage = sum(processes.map(p => p.cpuUsage)) - accumulatedCpuUsage;
        let otherMemoryUsage = sum(processes.map(p => p.memoryUsage)) - accumulatedMemoryUsage;
        tableHtml += `
            <tr>
                <td>...</td>
                <td>OTHER</td>
                <td>${formatPercent(otherCpuUsage)}</td>
                <td>${formatBytes(otherMemoryUsage)}</td>
            </tr>
        `;
    }

    tableData.innerHTML = tableHtml;
}

//bind the sort buttons
let cpuSortButton = document.getElementById("cpu-sort-button")!;
let nameSortButton = document.getElementById("name-sort-button")!;
let memSortButton = document.getElementById("mem-sort-button")!;
let pidSortButton = document.getElementById("pid-sort-button")!;

const UP_CHAR = "▲";
const DOWN_CHAR = "▼";

function sortClick(button: HTMLElement, type: "cpu" | "name" | "mem" | "pid") {
    if (procOrderType == type) {
        orderAscending = !orderAscending;
        if (orderAscending) {
            button.innerText = button.innerText.replace(DOWN_CHAR, UP_CHAR);
        } else {
            button.innerText = button.innerText.replace(UP_CHAR, DOWN_CHAR);
        }
    } else {
        let lastButton = document.getElementById(procOrderType + "-sort-button")!;
        procOrderType = type;
        orderAscending = false;
        lastButton.innerText = lastButton.innerText
            .replaceAll(UP_CHAR, "").replaceAll(DOWN_CHAR, "");
        button.innerText += DOWN_CHAR;
    }
    updateProcesses();
}

cpuSortButton.addEventListener("click", () => sortClick(cpuSortButton, "cpu"));
nameSortButton.addEventListener("click", () => sortClick(nameSortButton, "name"));
memSortButton.addEventListener("click", () => sortClick(memSortButton, "mem"));
pidSortButton.addEventListener("click", () => sortClick(pidSortButton, "pid"));




export async function updateSummary() {
    SUMMARY = await fetcher.getSummary();
    let summaryElement = document.getElementById("sys-summary");
    if (!summaryElement) {
        console.error("Could not find sys-summary element");
        return;
    }

    //find a summary child with id "hostname"
    let hostname = summaryElement.querySelector("#hostname")! as HTMLElement;
    hostname.innerText = "Hostname: \"" + SUMMARY.hostname + "\"";

    //find a summary child with id "os"
    let os = summaryElement.querySelector("#os")! as HTMLElement;
    os.innerText = "OS: \"" + SUMMARY.os + "\"";

    //find a summary child with id "cpu-cores"
    let cpuCores = summaryElement.querySelector("#cpu-cores")! as HTMLElement;
    cpuCores.innerText = "CPU Cores: " + SUMMARY.cpuCores;

    //find a summary child with id "total-memory"
    let totalMemory = summaryElement.querySelector("#total-memory")! as HTMLElement;
    totalMemory.innerText = "Total Memory: " + formatBytes(SUMMARY.totalMemory);


    let metadataElement = document.getElementById("metadata");
    if (!metadataElement) {
        console.error("Could not find metadata element");
        return;
    }

    let metaText = "";
    metaText += "Shiitake Version: " + SUMMARY.shiitakeVersion + " | ";
    metaText += "Webpage Version: " + SUMMARY.webpageVersion + " | ";
    metaText += "UUID: " + SUMMARY.uuid.toString(16);
    metadataElement.innerText = metaText;
}


