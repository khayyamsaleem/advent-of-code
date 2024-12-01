const std = @import("std");
const dotenv = @import("dotenv.zig");

const aoc = @import("./aoc.zig");
const day1 = @import("./01/solve.zig");

pub fn main() !void {
    var a = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = a.deinit();
    var gpa = a.allocator();

    try dotenv.load(gpa, .{});
    const sessionCookie = try std.process.getEnvVarOwned(gpa, "SESSION");
    defer gpa.free(sessionCookie);

    const puzzleInput = try aoc.fetchRawPuzzleInput(&gpa, sessionCookie, 2024, 1);
    defer gpa.free(puzzleInput);

    try day1.solve(&gpa, puzzleInput);
}
