const std = @import("std");

pub fn fetchRawPuzzleInput(a: std.mem.Allocator, cookie: []const u8, year: u16, day: u8) ![]const u8 {
    var client = std.http.Client{ .allocator = a };
    defer client.deinit();

    var endpoint_buf: [128]u8 = undefined;
    const endpoint = try std.fmt.bufPrint(&endpoint_buf, "https://adventofcode.com/{d}/day/{d}/input", .{ year, day });

    var cookie_buf: [256]u8 = undefined;
    const cookieHeaderValue = try std.fmt.bufPrint(&cookie_buf, "session={s}", .{cookie});

    const headers = [_]std.http.Header{
        .{ .name = "cookie", .value = cookieHeaderValue },
    };

    var body_storage = std.io.Writer.Allocating.init(a);
    defer body_storage.deinit();

    const fetch_result = try client.fetch(.{
        .location = .{ .url = endpoint },
        .method = .GET,
        .extra_headers = &headers,
        .response_writer = &body_storage.writer,
    });

    if (fetch_result.status != .ok) {
        std.debug.print("got a non-200 status: {}\n", .{fetch_result.status});
        return error.ResponseNotOk;
    }

    const body = try body_storage.toOwnedSlice();
    return body;
}
