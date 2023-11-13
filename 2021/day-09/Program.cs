var seaFloor = new SeaFloor(File.ReadAllLines(@"./input.txt"));

var lowPoints = seaFloor.LowPoints();

Console.WriteLine($"Part One: {lowPoints.Select(point => seaFloor.Depth(point) + 1).Sum()}");

var partTwo = lowPoints.Select(low => seaFloor.BasinSize(low))
                          .OrderByDescending(item => item)
                          .Take(3)
                          .Aggregate(1, (acc, next) => acc * next);

Console.WriteLine($"Part two: {partTwo}");