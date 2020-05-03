import XCTest

#if !canImport(ObjectiveC)
public func allTests() -> [XCTestCaseEntry] {
    return [
        testCase(aoc_2019_day_12Tests.allTests),
    ]
}
#endif
