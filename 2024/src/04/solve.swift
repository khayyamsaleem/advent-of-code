import Foundation

func partOne(_ input: String) -> Int {
    // TODO: Implement part 1 solution
    let lines = input.components(separatedBy: .newlines).filter { !$0.isEmpty }
    return lines.count
}

func partTwo(_ input: String) -> Int {
    // TODO: Implement part 2 solution
    return 0
}

func solveInner(_ input: String) {
    let result1 = partOne(input)
    print("Day 04 - Part 1: \(result1)")

    let result2 = partTwo(input)
    print("Day 04 - Part 2: \(result2)")
}

@_cdecl("solve")
public func solve(_ inputPtr: UnsafePointer<CChar>?) {
    guard let inputPtr = inputPtr else {
        fputs("Error: null input pointer\n", stderr)
        return
    }

    let input = String(cString: inputPtr)
    solveInner(input)
}
