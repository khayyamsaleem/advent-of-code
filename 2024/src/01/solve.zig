const std = @import("std");

fn zsolve(a: std.mem.Allocator, input: []const u8) !void {
    var stdout_buffer: [4096]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    var left = try std.ArrayList(i32).initCapacity(a, 0);
    var right = try std.ArrayList(i32).initCapacity(a, 0);
    defer left.deinit(a);
    defer right.deinit(a);

    var iter = std.mem.splitScalar(u8, input, '\n');
    while (iter.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        var parts = std.mem.splitSequence(u8, line, "   ");
        const l = std.fmt.parseInt(i32, parts.next() orelse "0", 10) catch return error.InvalidInput;
        const r = std.fmt.parseInt(i32, parts.next() orelse "0", 10) catch return error.InvalidInput;
        try left.append(a, l);
        try right.append(a, r);
    }

    std.mem.sort(i32, left.items, {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, right.items, {}, comptime std.sort.asc(i32));

    const length = left.items.len;

    var diffSum: u32 = 0;
    for (0..length) |i| {
        diffSum += @abs(left.items[i] - right.items[i]);
    }
    try stdout.print("Day 01 - Part 1: {d}\n", .{diffSum});

    var similarityScore: u32 = 0;
    var rp: u32 = 0;
    for (0..length) |i| {
        var c: u32 = 0;

        while (rp < length and left.items[i] > right.items[rp]) {
            rp += 1;
        }

        while (rp < length and left.items[i] == right.items[rp]) {
            c += 1;
            rp += 1;
        }

        similarityScore += c * @abs(left.items[i]);
    }

    try stdout.print("Day 01 - Part 2: {d}\n", .{similarityScore});
    try stdout.flush();
}

pub export fn solve(input: [*:0]const u8) void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const a = gpa.allocator();

    zsolve(a, std.mem.span(input)) catch |err| {
        std.log.err("unable to solve: {}", .{err});
        return;
    };
}
