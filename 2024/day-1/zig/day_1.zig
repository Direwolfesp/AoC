const std = @import("std");
const N_LINES = 1000;

fn part_a(filepath: []const u8) !u32 {
    var file = try std.fs.cwd().openFile(filepath, .{});
    defer file.close();
    var buf: [4096]u8 = undefined;
    var reader = file.readerStreaming(&buf);

    var list_l: [N_LINES]i32 = undefined;
    var list_r: [N_LINES]i32 = undefined;
    var n_lines: u64 = 0;

    while (try reader.interface.takeDelimiter('\n')) |line| : (n_lines += 1) {
        list_l[n_lines] = try std.fmt.parseInt(i32, line[0..5], 10);
        list_r[n_lines] = try std.fmt.parseInt(i32, line[8..13], 10);
    }

    std.mem.sort(i32, list_l[0..n_lines], {}, std.sort.asc(i32));
    std.mem.sort(i32, list_r[0..n_lines], {}, std.sort.asc(i32));

    var total: u32 = 0;
    for (0..n_lines) |i| total += @abs(list_l[i] - list_r[i]);
    return total;
}

pub fn main() !void {
    if (std.os.argv.len != 2) {
        std.debug.print("[!] missing filepath\n", .{});
        return;
    }

    const filepath: []const u8 = std.mem.span(std.os.argv[1]);
    std.debug.print("> {d}\n", .{try part_a(filepath)});
}
