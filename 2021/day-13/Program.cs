public static class Program
{
    public static void Main(string[] args)
    {
        var file = Lib.FileReader.ReadLines("13");
        var points = file.TakeWhile(s => s != "").Select(s => s.Split(',')).Select(s => new Point(int.Parse(s[0]), int.Parse(s[1]))).ToHashSet();
        var instructions = file.Skip(points.Count + 1).ToList();


        var partOne = points.Mirror(instructions[0]).Count();

        Console.WriteLine($"Part One: {partOne}");

        var partTwo = instructions.Aggregate(points, (acc, instruction) => acc.Mirror(instruction)) ?? new();
        var graph = new Graph(partTwo);
        Console.WriteLine("Part Two:");
        graph.WriteToConsole();
    }

    public static HashSet<Point> Mirror(this IEnumerable<Point> points, string instruction)
    {
        var instructionPair = instruction.Substring(11).Split('=').ToList();
        Func<Point, Point> func = instructionPair[0] switch
        {
            "x" => (point) =>
            {
                var xFold = int.Parse(instructionPair[1]);
                if (point.X > xFold)
                {
                    return point with { X = xFold - (point.X - xFold) };
                }
                else
                {
                    return point;
                }
            }
            ,

            "y" => (point) =>
            {
                var yFold = int.Parse(instructionPair[1]);
                if (point.Y > yFold)
                {
                    return point with { Y = yFold - (point.Y - yFold) };
                }
                else
                {
                    return point;
                }
            }
            ,

            _ => throw new ArgumentException("Invalid instruciton")
        };

        return points.Select(func)
            .ToHashSet();
    }
}

public record Point(int X, int Y) { }

public class Graph
{
    public Point?[,] Plane { get; init; }

    public Graph(IEnumerable<Point> points)
    {
        var xMax = points.MaxBy(p => p.X)?.X + 1 ?? 0;
        var yMax = points.MaxBy(p => p.Y)?.Y + 1 ?? 0;
        Plane = new Point?[yMax, xMax];
        foreach (var point in points)
        {
            Plane[point.Y, point.X] = point;
        }
    }

    public void WriteToConsole()
    {
        for (int i = 0; i < Plane.GetLength(0); i++)
        {
            for (int j = 0; j < Plane.GetLength(1); j++)
            {
                Console.Write(Plane[i, j] is null ? ' ' : '#');
            }
            Console.Write('\n');
        }
    }
}
