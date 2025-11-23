const std = @import("std");
// const dotenv = @import("dotenv.zig"); // Disabled due to hanging issue

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

    var libname_buf: [32]u8 = undefined;
    const libname = try std.fmt.bufPrint(&libname_buf, "libsolve{:0>2}.so", .{moduleNum});

    const libprefix_owned = std.process.getEnvVarOwned(a, "AOC_LIB_PATH") catch null;
    defer if (libprefix_owned) |p| a.free(p);
    const libprefix = libprefix_owned orelse "../lib";

    const libpath = try std.fs.path.join(a, &.{ libprefix, libname });
    defer a.free(libpath);

    var lib = std.DynLib.open(libpath) catch |err| {
        std.log.err("couldn't find it at {s}: {}", .{ libpath, err });
        return err;
    };
    defer {
        lib.close();
    }

    const solve_fn = *const fn (input: [*c]const u8) callconv(.c) void;
    const solve = lib.lookup(solve_fn, "solve") orelse return error.FunctionNotFound;

    const sessionCookie = std.process.getEnvVarOwned(a, "SESSION") catch blk: {
        // Read .env file manually
        const env_file = try std.fs.cwd().openFile(".env", .{});
        defer env_file.close();

        const contents = try env_file.readToEndAlloc(a, 1024 * 1024);
        defer a.free(contents);

        var lines = std.mem.splitScalar(u8, contents, '\n');
        while (lines.next()) |line| {
            if (std.mem.startsWith(u8, line, "SESSION=")) {
                const value = std.mem.trim(u8, line[8..], &std.ascii.whitespace);
                break :blk try a.dupe(u8, value);
            }
        }
        return error.SessionNotFound;
    };
    defer a.free(sessionCookie);

    const puzzleInput = try aoc.fetchRawPuzzleInput(a, sessionCookie, 2024, moduleNum);
    defer a.free(puzzleInput);

    // Convert to null-terminated slice for the solve function
    const i = try a.dupeZ(u8, puzzleInput);
    defer a.free(i);

    solve(@ptrCast(i.ptr));
}
