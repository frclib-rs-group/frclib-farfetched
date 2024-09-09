import '@picocss/pico/css/pico.classless.min.css';
import * as webpage from './webpage';

let cycleCount = 0;

/**
 * A function that will be looped every 50ms
 */
async function cycle() {
    webpage.updateUptime();
    if (cycleCount % 20 == 0) {
        webpage.updateResources();
    }
    if (cycleCount % 60 == 0) {
        webpage.updateProcesses();
    }
    if (cycleCount % 100 == 0) {
        webpage.updateSummary();
    }

    cycleCount++;
}

webpage.updateUptime();
webpage.updateSummary();
webpage.updateResources();
webpage.updateProcesses();

while (true) {
    await new Promise(resolve => setTimeout(resolve, 50));
    await cycle();
}
