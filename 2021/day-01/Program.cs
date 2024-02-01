var file = Lib.FileReader.ReadLines("01").Select(item => int.Parse(item)).ToArray();

var part1 = file.Skip(1)
                .Zip(file)
                .Where(tuple => tuple.First > tuple.Second)
                .Count();

var sumOf3 = file.Skip(2)
                 .Zip(file.Skip(1), file)
                 .Select(triple => triple.First + triple.Second + triple.Third)
                 .ToArray();

var part2 = sumOf3.Skip(1)
                  .Zip(sumOf3)
                  .Where(tuple => tuple.First > tuple.Second)
                  .Count();

Console.WriteLine($"Part One: {part1}");
Console.WriteLine($"Part Two: {part2}");
