var file = File.ReadAllLines(@"./input.txt");

var graph = new Graph(file);

var paths = graph.Paths();

Console.WriteLine($"Part One: {paths.Count()}");

var paths2 = graph.Paths(true);

Console.WriteLine($"Part Two: {paths2.Count()}");