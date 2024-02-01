var file = Lib.FileReader.ReadLines("12");

var graph = new Graph(file);

var paths = graph.Paths();

Console.WriteLine($"Part One: {paths.Count()}");

var paths2 = graph.Paths(true);

Console.WriteLine($"Part Two: {paths2.Count()}");
