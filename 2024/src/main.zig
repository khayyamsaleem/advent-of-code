const std = @import("std");
const dotenv = @import("dotenv.zig");

const aoc = @import("./aoc.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{
        .safety = true,
        .never_unmap = true,
        .retain_metadata = true,
    }){};
    defer {
        _ = gpa.deinit();
    }
    var a = gpa.allocator();

    const args = try std.process.argsAlloc(a);
    defer std.process.argsFree(a, args);

    if (args.len != 2) {
        std.log.err("Usage: {s} <module>", .{args[0]});
        return;
    }

    const moduleNum = try std.fmt.parseInt(u8, args[1], 10);
    if (moduleNum == 0 or moduleNum > 25) {
        std.log.err("must supply a valid day", .{});
        return;
    }

    const libname = try std.fmt.allocPrint(a, "libsolve{:0>2}.so", .{moduleNum});
    defer a.free(libname);

    const libprefix = std.process.getEnvVarOwned(std.heap.page_allocator, "AOC_LIB_PATH") catch "../lib";

    const libpath = try std.fs.path.join(a, &.{ libprefix, libname });
    defer a.free(libpath);

    var lib = std.DynLib.open(libpath) catch |err| {
        std.log.err("couldn't find it at {s}: {}", .{ libpath, err });
        return err;
    };
    defer {
        lib.close();
    }

    const solve_fn = *const fn (input: [*:0]const u8) void;
    const solve = lib.lookup(solve_fn, "solve") orelse return error.FunctionNotFound;

    try dotenv.load(a, .{});
    const sessionCookie = try std.process.getEnvVarOwned(a, "SESSION");
    defer a.free(sessionCookie);

    const puzzleInput = try aoc.fetchRawPuzzleInput(a, sessionCookie, 2024, moduleNum);
    defer {
        a.free(puzzleInput);
    }

    const i = try a.dupeZ(u8, puzzleInput);
    defer a.free(i);

    solve(i);
}
