// couldn't figure out how to make a request with `https` lib so used this one
const request = require('request-promise')
const fs = require('fs')
const path = require('path')
const { Worker } = require("worker_threads")
const pkg = require("./package.json")

const USER_AGENT = `node/${process.version} ${pkg.name}/${pkg.version}`

// stores session cookie as: session=<cookie>
const dotEnvFile = path.join(__dirname, '.env')

// path to path creation worker
// workers are used so that paths for wire1 and wire2 can be made concurrently
// scales for n wires
const wireWorkerScript = path.join(__dirname, "wirePathWorker.js")

// creates a Promise for worker completion
const createWorkerForWire = (wire) => {
    return new Promise((res, rej) => {
        const worker = new Worker(wireWorkerScript, {workerData: wire})
        worker.on("message", res)
        worker.on("error", rej)
    })
}

// determines the intersection of two lists
const intersection = (a, b) =>  {
    var t;
    if (b.length > a.length) t = b, b = a, a = t;
    return a.filter((e) => b.indexOf(e) > -1)
}

// retrieves input from AOC for day 3
const getInput = (sessionCookie) => {
    return request({
        method: "GET",
        uri: "http://adventofcode.com/2019/day/3/input",
        headers: {
            "Cookie": `session=${sessionCookie}`,
            "User-Agent": USER_AGENT
        }
    })
}

// computes part one answer given intersections
const partOne = (crosses) => {
    return Math.min(...(crosses.map(([x,y]) => Math.abs(x) + Math.abs(y))))
}

// computes part two answer given steps wires took and intersections of wires
const partTwo = (wirePaths, crosses) => {
    const [wire1, wire2] = wirePaths
    let dists = []
    for (let cross of crosses) {
        w1dist = 0
        while (`${cross}` !== `${wire1[w1dist]}` ) {
            w1dist++
        }
        w2dist = 0
        while (`${cross}` !== `${wire2[w2dist]}`) {
            w2dist++
        }
        // add extra two to account for first step from central port for each wire
        dists.push(w1dist + w2dist + 2)
    }
    return Math.min(...dists)
}

// reads the dotenv file
fs.readFile(dotEnvFile, {encoding: 'utf-8'}, async (err, data) => {
    if (err) {
        console.log(err)
    } else {
        const [_, sessionCookie ] = data.trim().split('=')
        const res = await getInput(sessionCookie)

        // turns wires into [['<DIR:char>', <STEPS:int>], ...]
        const wires = res.trim().split('\n').map(s =>
            s.split(',').map(op => [op[0], parseInt(op.slice(1))])
        )

        const [wire1path, wire2path] = await Promise.all(wires.map(createWorkerForWire))

        const crosses = intersection(wire1path, wire2path).map(c => c.split(','))

        console.log(partOne(crosses))

        console.log(partTwo([wire1path, wire2path], crosses))
    }
})

