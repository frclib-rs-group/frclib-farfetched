import * as d3 from 'd3';

const COLORS = [
    "#ff0000",
    "#0000ff",
    "#00ff00",
    "#ffff00",
    "#00ffff",
    "#ff8000",
    "#80ff00",
    "#00ff80",
    "#0080ff",
    "#800000",
    "#000080",
    "#008000",
    "#808000",
    "#008080",
    "#804000",
    "#408000",
    "#008040",
    "#004080",
]

export function putGraph(
    container: HTMLDivElement,
    data: (number[] | null)[],
    keySupplier: (idx: number) => string,
    historyDuration: number,
    bounds: [number, number],
    monoColor: boolean | string
    ) {
    //input validation
    if (data.length == 0) {
        console.error("Cannot put graph with no data");
        return;
    }
    if (historyDuration < 0) {
        console.error("Cannot put graph with negative historyDuration");
        return;
    }

    //make sure every number[] in data is the same length or all null
    let numbers = data.filter(d => d != null) as number[][];
    let allNull = numbers.length == 0;
    if (numbers.length > 0) {
        let length = numbers[0]!.length;
        for (let i = 1; i < numbers.length; i++) {
            if (numbers[i]!.length != length) {
                console.error("Cannot put graph with data of different lengths");
                return;
            }
        }
    }

    let lineCount = numbers[0].length;

    if (lineCount > COLORS.length) {
        console.error("Cannot put graph with more lines than colors");
        return;
    }

    if (lineCount == 0) {
        allNull = true;
        data = data.map(d => null);
    }


    let width = container.clientWidth;
    let height = window.innerWidth < 600 ? window.innerHeight / 5 : window.innerHeight / 3;
    // let margin = {top: 20, right: 20, bottom: 30, left: 50};

    //make x go from -historyDuration to 0
    const x = d3.scaleLinear()
        .domain([-historyDuration, 0])
        .range([0, width]);

    //make y go from 0 to 100
    const y = d3.scaleLinear()
        .domain(bounds)
        .range([height, 0]);

    //a line for the numbers we have
    const activeLine = d3.line<number>()
        .defined(d => !isNaN(d))
        .x((d, i) => x(i - data.length))
        .y(d => y(d!));

    const svg = d3.create("svg")
        .attr("width", width)
        .attr("height", height)
        .attr("viewBox", [0, 0, width, height])
        .attr("style", "max-width: 100%; height: auto; height: intrinsic;");


    //add the x axis
    svg.append("g")
        .attr("transform", "translate(0," + height + ")")
        .call(d3.axisBottom(x));

    //add the y axis
    svg.append("g")
        .call(d3.axisLeft(y));

    const keyDiv = document.createElement("p");
    keyDiv.style.display = "flex";
    keyDiv.style.flexDirection = "row";
    keyDiv.style.flexWrap = "wrap";
    keyDiv.style.justifyContent = "center";
    keyDiv.style.alignItems = "center";

    let getColor = (i: number) => {
        if (monoColor) {
            if (typeof monoColor == "string") {
                return monoColor;
            } else {
                return "#ffffff";
            }
        } else {
            return COLORS[i];
        }
    }

    if (allNull) {
        svg.append("path")
            .attr("fill", "none")
            .attr("stroke", "red")
            .attr("stroke-width", 2.0)
            //@ts-ignore
            .attr("d", inactiveLine(data));
    } else {
        for (let i = 0; i < lineCount; i++) {
            svg.append("path")
                .attr("fill", "none")
                .attr("stroke", getColor(i))
                .attr("stroke-width", 2.0)
                //@ts-ignore
                .attr("d", activeLine(data.map(d => d![i])));

            let key = document.createElement("h7");
            key.style.margin = "0.5em";
            key.innerText = keySupplier(i);
            key.style.color = getColor(i);
            keyDiv.appendChild(key);
        }
    }

    //clear the container and put the svg in it
    container.innerHTML = "";
    container.appendChild(svg.node()!);
    container.appendChild(keyDiv);
}

export function putGraphSingle(
    container: HTMLDivElement,
    data: (number | null)[],
    keySupplier: (idx: number) => string,
    historyDuration: number,
    bounds: [number, number],
    monoColor: boolean | string
    ) {
    putGraph(
        container,
        data.map(d => d == null ? null : [d]),
        keySupplier, historyDuration,
        bounds,
        monoColor
    );
}