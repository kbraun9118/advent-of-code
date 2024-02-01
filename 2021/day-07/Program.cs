var input = Lib.FileReader.ReadLines("07")[0].Split(",").Select(item => int.Parse(item)).ToArray();

var max = input.Max();
var min = input.Min();

var range = Enumerable.Range(min, max - min + 1);

var partOne = range.Select(position => input.Select(crab => Math.Abs(position - crab)).Sum())
                .Min();

Console.WriteLine($"Part One: {partOne}");

var stepFuelCost = (int distance) =>
{
    return distance * ((distance + 1) / 2) + (distance % 2 == 0 ? distance / 2 : 0);
};

var partTwo = range.Select(position => input.Select(crab => stepFuelCost(Math.Abs(position - crab))).Sum())
                   .Min();

Console.WriteLine($"Part Two: {partTwo}");
