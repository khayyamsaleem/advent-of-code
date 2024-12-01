const std = @import("std");

pub fn fetchRawPuzzleInput(a: *std.mem.Allocator, cookie: []const u8, year: u16, day: u8) ![]u8 {
    var client = std.http.Client{ .allocator = a.* };
    defer client.deinit();

    const endpoint = try std.fmt.allocPrint(a.*, "https://adventofcode.com/{d}/day/{d}/input", .{ year, day });
    defer a.free(endpoint);

    const cookieHeaderValue = try std.fmt.allocPrint(a.*, "session={s}", .{cookie});
    defer a.free(cookieHeaderValue);

    const headers = [_]std.http.Header{
        .{ .name = "cookie", .value = cookieHeaderValue },
    };

    var body = std.ArrayList(u8).init(a.*);
    defer body.deinit();

    const res = try client.fetch(.{
        .location = .{ .url = endpoint },
        .method = .GET,
        .extra_headers = &headers,
        .response_storage = .{ .dynamic = &body },
    });

    if (res.status == .ok) {
        return body.toOwnedSlice();
    }

    std.debug.print("got a non-200 status: {d}\n", .{res.status});

    return error.ResponseNotOk;
}
