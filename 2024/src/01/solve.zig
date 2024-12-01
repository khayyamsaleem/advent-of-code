const std = @import("std");

pub fn solve(a: *std.mem.Allocator, input: []u8) !void {
    var lines = std.ArrayList([]const u8).init(a.*);
    defer lines.deinit();

    var iter = std.mem.split(u8, input, "\n");
    while (iter.next()) |line| {
        try lines.append(line);
    }

    var left = std.ArrayList(i32).init(a.*);
    var right = std.ArrayList(i32).init(a.*);
    defer left.deinit();
    defer right.deinit();

    for (lines.items) |line| {
        if (line.len == 0) {
            continue;
        }
        var parts = std.mem.split(u8, line, "   ");
        const l = std.fmt.parseInt(i32, parts.next() orelse "0", 10) catch return error.InvalidInput;
        const r = std.fmt.parseInt(i32, parts.next() orelse "0", 10) catch return error.InvalidInput;
        try left.append(l);
        try right.append(r);
    }

    std.mem.sort(i32, left.items, {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, right.items, {}, comptime std.sort.asc(i32));

    const length = left.items.len;

    var diffSum: u32 = 0;
    for (0..length) |i| {
        diffSum += @abs(left.items[i] - right.items[i]);
    }
    try std.io.getStdOut().writer().print("Day 01 - Part 1: {d}\n", .{diffSum});

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

    try std.io.getStdOut().writer().print("Day 01 - Part 2: {d}\n", .{similarityScore});
}
