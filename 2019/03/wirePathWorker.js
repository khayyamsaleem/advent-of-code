const {parentPort, workerData: wire} = require("worker_threads")

let coords = []
let current = [0, 0]
for (let i = 0; i < wire.length; i++) {
    const [x, y] = current;
    const [dir, steps] = wire[i]
    switch (dir) {
        case 'U':
            for (let j = 1; j <= steps; j++)
                coords.push(`${[x, y+j]}`)
            current = [x, y+steps]
            break;
        case 'R':
            for (let j = 1; j <= steps; j++)
                coords.push(`${[x+j, y]}`)
            current = [x+steps,y]
            break;
        case 'L':
            for (let j = 1; j <= steps; j++)
                coords.push(`${[x-j, y]}`)
            current = [x-steps, y]
            break;
        case 'D':
            for (let j = 1; j <= steps; j++)
                coords.push(`${[x, y-j]}`)
            current = [x, y-steps]
            break;
        default:
            throw new Error("Invalid direction")
    }
}

parentPort.postMessage(coords)
