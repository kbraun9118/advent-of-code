var file = Lib.FileReader.ReadLines("05");

var lines = file.Select(line => new Line(line)).ToList();

var maxX = lines.Select(line => line.MaxX()).Max();
var maxY = lines.Select(line => line.MaxY()).Max();

var map = new Map(maxX + 1, maxY + 1);

foreach (var line in lines.Where(line => line.IsHorizontal() || line.IsVertical()))
{
    map.AddToMap(line.PointsBetween());
}

Console.WriteLine($"Part One: {map.Overlap()}");

map.Clear();

foreach (var line in lines)
{
    map.AddToMap(line.PointsBetween());
}

Console.WriteLine($"Part Two: {map.Overlap()}");
