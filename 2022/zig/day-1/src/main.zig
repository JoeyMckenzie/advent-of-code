const std = @import("std");

pub fn main() !void {
    var file = try std.fs.cwd().openFile("file.txt", .{});
defer file.close();

var buf_reader = std.io.bufferedReader(file.reader());
var in_stream = buf_reader.reader();

var buf: [1024]u8 = undefined;
while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
    _ = line;
    // do something with line...
}

}
