import Foundation
import Alamofire
import DotEnv

let env = DotEnv(withFile: ".env")

// GCD of a vector of numbers:
func gcd(_ vector: [Int]) -> Int {

    func gcd(_ a: Int, _ b: Int) -> Int {
        var (a, b) = (a, b)
        while b != 0 {
            (a, b) = (b, a % b)
        }
        return abs(a)
    }
    return vector.reduce(0, gcd)
}


// LCM of a vector of numbers:
func lcm(_ vector : [Int]) -> Int {
    func lcm(a: Int, b: Int) -> Int { (a / gcd([a, b])) * b }
    return vector.reduce(1, lcm)
}

extension String {
    func groups(for regexPattern: String) -> [[String]] {
        do {
            let text = self
            let regex = try NSRegularExpression(pattern: regexPattern)
            let matches = regex.matches(in: text,
                                        range: NSRange(text.startIndex..., in: text))
            return matches.map { match in
                return (0..<match.numberOfRanges).map {
                    let rangeBounds = match.range(at: $0)
                    guard let range = Range(rangeBounds, in: text) else {
                        return ""
                    }
                    return String(text[range])
                }
            }
        } catch let error {
            print("invalid regex: \(error.localizedDescription)")
            return []
        }
    }
}

extension Int {
    func compareTo(for other: Int) -> Int {
        if (self - other) < 0 { return 1 }
        if (self - other) == 0 { return 0 }
        return -1 
    } 
}


public struct Moon : Hashable {
    public var position = (x: 0, y: 0, z: 0);
    public var velocity = (x: 0, y: 0, z: 0);

    public func hash(into hasher : inout Hasher) {
        hasher.combine(self.position.x)
    }

    public static func == (lhs: Moon, rhs: Moon) -> Bool {
        return lhs.position.x == rhs.position.x && lhs.position.y == rhs.position.y && lhs.position.z == rhs.position.z &&
               lhs.velocity.x == rhs.velocity.x && lhs.velocity.y == rhs.velocity.y && lhs.velocity.z == rhs.velocity.z
    }

    public func computePotentialEnergy() -> Int {
        return abs(position.x) + abs(position.y) + abs(position.z)
    }

    public func computeKineticEnergy() -> Int {
        return abs(velocity.x) + abs(velocity.y) + abs(velocity.z)
    }

    public func computeTotalEnergy() -> Int {
        return self.computePotentialEnergy() * self.computeKineticEnergy()
    }

    public func getSignatureForDimension(dimension: String) -> Pair {
        switch dimension {
            case "x": 
                return Pair(a: self.position.x, b :self.velocity.x)
            case "y": 
                return Pair(a: self.position.y, b :self.velocity.y)
            case "z": 
                return Pair(a: self.position.z, b :self.velocity.z)
            default:
                fatalError("Invalid dimension")
        }
    }
}

extension Moon : CustomStringConvertible {
    public var description: String {
        return "[ position = \(position), velocity = \(velocity) ]"
    }
}


func getPuzzleInput(url: String, completion: @escaping (AFResult<[String]>) -> Void) {
    let headers: HTTPHeaders = [
        "Cookie": "session=\(env.get("session")!)"
    ]
    AF.request(url, headers: headers).responseString { response in
        switch response.result {
        case .success(let puzzleInput):
            completion(.success(puzzleInput
                    .trimmingCharacters(in:.whitespacesAndNewlines)
                    .components(separatedBy: "\n"))
            )
        case .failure(let error):
            completion(.failure(error))
        }
    }
}

public func parseMoonExpr(moonExpr: String) -> Moon {
    let pattern = "^<x=(?<xPos>-?\\d*), y=(?<yPos>-?\\d*), z=(?<zPos>-?\\d*)>"
    let groups = moonExpr.groups(for: pattern)[0]
    return Moon(position: (x: Int(groups[1])!, y: Int(groups[2])!, z: Int(groups[3])!))
}

public func applyGravityForPair(moon: Moon, otherMoon: Moon) -> Moon {
    let newMoon = Moon(
        position: moon.position,
        velocity: (
            x: moon.velocity.x + moon.position.x.compareTo(for: otherMoon.position.x),
            y: moon.velocity.y + moon.position.y.compareTo(for: otherMoon.position.y),
            z: moon.velocity.z + moon.position.z.compareTo(for: otherMoon.position.z)
        )
    )
    return newMoon
}

public func applyVelocity(moon: Moon) -> Moon {
    Moon(
        position: (
            x: moon.position.x + moon.velocity.x,
            y: moon.position.y + moon.velocity.y,
            z: moon.position.z + moon.velocity.z
        ),
        velocity: moon.velocity
    )
}

public func oneSmallStepForMoon(_ moons: [Moon]) -> [Moon] {
    moons.map({moon in 
        applyVelocity(moon: moons.reduce(moon, applyGravityForPair))
    })
}

public func calculateTotalEnergyInSystem(_ moons: [Moon]) -> Int {
    moons.reduce(0, {acc, moon in
        acc + moon.computeTotalEnergy()
    })
}

func iterate<T: Any>(_ f : (T) -> T, _ n: Int, _ e: T) -> T {
    return n == 0 ? e : iterate(f, n-1, f(e))
}


public struct Pair: Hashable {
    let a: Int
    let b: Int
}

public func getFirstRepeatForDimension(_ moons: [Moon], _ dimension: String) -> Int {
    var seen = Set<[Pair]>()
    var moonState = moons
    let initialSignatureForDimension = moonState.map({moon in moon.getSignatureForDimension(dimension: dimension)})
    var count = 0
    while !seen.contains(initialSignatureForDimension) {
       count += 1 
       moonState = oneSmallStepForMoon(moonState)
       seen.insert(moonState.map({moon in moon.getSignatureForDimension(dimension: dimension)}))
    }
    return count
}

func partOne(_ moons: [Moon]) -> Int {
    calculateTotalEnergyInSystem(iterate(oneSmallStepForMoon, 1000, moons))
}

func partTwo(_ moons: [Moon]) -> Int {
    lcm(["x", "y", "z"].map({dim in getFirstRepeatForDimension(moons, dim)}))
}

func solution(maybeArrayOfMoonExprs: AFResult<[String]>) -> Void {
    switch maybeArrayOfMoonExprs {
        case .success(let moonExprs):
            let moons = moonExprs.map(parseMoonExpr)
            print("Part One: \(partOne(moons))")
            print("Part Two: \(partTwo(moons))")
        case .failure(let error):
            fatalError("Failed to get moons from puzzle input: \(error)")
    }
    exit(0)
}


func main() {
    getPuzzleInput(url: "https://adventofcode.com/2019/day/12/input", completion: solution)
}

main()



RunLoop.main.run()
