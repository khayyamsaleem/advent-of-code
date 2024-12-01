const std = @import("std");
const dotenv = @import("dotenv.zig");

const aoc = @import("./aoc.zig");

pub fn main() !void {
    var a = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = a.deinit();
    var gpa = a.allocator();

    try dotenv.load(gpa, .{});
    const sessionCookie = try std.process.getEnvVarOwned(gpa, "SESSION");
    defer gpa.free(sessionCookie);

    const stdout = std.io.getStdOut();

    const puzzleInput = try aoc.fetchRawPuzzleInput(&gpa, sessionCookie, 2024, 1);
    defer gpa.free(puzzleInput);

    try stdout.writer().print("input:\n{s}\n", .{puzzleInput});
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
