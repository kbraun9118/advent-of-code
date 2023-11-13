var file = File.ReadAllLines(@"./input.txt");

var scanners = file
    .ChunkWhile(s => s.StartsWith("--") is false)
    .Select(s => s
        .Skip(1)
        .Where(s => s.Length > 0)
        .Aggregate(
            new Scanner(),
            (acc, next) =>
            {
                acc.Points.Add(new Point(next));
                return acc;
            })
    ).ToList();

var baseScanner = new HashSet<Point>(scanners[0].Points);
scanners.RemoveAt(0);
var scannerLocations = new List<Point>() { new Point(0, 0, 0) };

while (scanners.Count > 0)
{
    var scannerCenter = FindScanner(scanners, baseScanner);
    if (scannerCenter is Point location) scannerLocations.Add(location);
}

Console.WriteLine($"Part One: {baseScanner.Count}");

var partTwo = from l in scannerLocations
              from r in scannerLocations
              select l.Distance(r);

Console.WriteLine($"Part Two: {partTwo.Max()}");

Point? FindScanner(List<Scanner> scanners, HashSet<Point> baseScanner)
{
    for (int i = 0; i < scanners.Count; i++)
    {
        foreach (var orientation in scanners[i].Orientations())
        {
            foreach (var basePoint in baseScanner)
            {
                foreach (var orientationPoint in orientation.Points)
                {
                    var translation = (basePoint.X - orientationPoint.X, basePoint.Y - orientationPoint.Y, basePoint.Z - orientationPoint.Z);
                    var translatedOrientation = orientation.Translate(translation);
                    var intersection = translatedOrientation.Points.Intersect(baseScanner).Count();
                    if (intersection >= 12)
                    {
                        baseScanner.UnionWith(translatedOrientation.Points);
                        scanners.RemoveAt(i);
                        return new Point(translation.Item1, translation.Item2, translation.Item3);
                    }
                }
            }
        }
    }
    return null;
}

public class Scanner
{
    public HashSet<Point> Points { get; init; } = new();

    public Scanner(IEnumerable<Point> points)
    {
        Points = points.ToHashSet();
    }

    public Scanner()
    {
    }

    public List<Scanner> Orientations()
    {
        var points = Points
            .Select(p => p.AllOrientations())
            .ToList();

        List<Scanner> scanners = new();
        for (int i = 0; i < 24; i++)
        {
            scanners.Add(new Scanner(points.Select(p => p[i])));
        }
        return scanners;
    }

    internal Scanner Translate((int X, int Y, int Z) translation)
    {
        return new Scanner(Points.Select(p => new Point(p.X + translation.X, p.Y + translation.Y, p.Z + translation.Z)));
    }
}

public record Point(int X, int Y, int Z)
{

    public Point(string input) : this(0, 0, 0)
    {
        var split = input.Split(',');
        X = int.Parse(split[0]);
        Y = int.Parse(split[1]);
        Z = int.Parse(split[2]);
    }

    public List<Point> AllOrientations()
    {

        /*
            0 => ,
        */
        return new()
        {
            this with { X = X, Y = Y, Z = Z },
            this with { X = Y, Y = Z, Z = X },
            this with { X = -Y, Y = X, Z = Z },
            this with { X = -X, Y = -Y, Z = Z },
            this with { X = Y, Y = -X, Z = Z },
            this with { X = Z, Y = Y, Z = -X },
            this with { X = Z, Y = X, Z = Y },
            this with { X = Z, Y = -Y, Z = X },
            this with { X = Z, Y = -X, Z = -Y },
            this with { X = -X, Y = Y, Z = -Z },
            this with { X = Y, Y = X, Z = -Z },
            this with { X = X, Y = -Y, Z = -Z },
            this with { X = -Y, Y = -X, Z = -Z },
            this with { X = -Z, Y = Y, Z = X },
            this with { X = -Z, Y = X, Z = -Y },
            this with { X = -Z, Y = -Y, Z = -X },
            this with { X = -Z, Y = -X, Z = Y },
            this with { X = X, Y = -Z, Z = Y },
            this with { X = -Y, Y = -Z, Z = X },
            this with { X = -X, Y = -Z, Z = -Y },
            this with { X = Y, Y = -Z, Z = -X },
            this with { X = X, Y = Z, Z = -Y },
            this with { X = -Y, Y = Z, Z = -X },
            this with { X = -X, Y = Z, Z = Y },
        };
    }

    internal int Distance(Point r)
    {
        return Math.Abs(X - r.X) + Math.Abs(Y - r.Y) + Math.Abs(Z - r.Z);
    }
}

public static class Extensions
{
    public static IEnumerable<TSource[]> ChunkWhile<TSource>(this IEnumerable<TSource> enumerable, Func<TSource, bool> func)
    {
        using var enumerator = enumerable.GetEnumerator();
        var collection = new List<TSource>();
        if (enumerator.MoveNext()) collection.Add(enumerator.Current);
        while (enumerator.MoveNext())
        {
            if (func(enumerator.Current))
            {
                collection.Add(enumerator.Current);
            }
            else
            {
                yield return collection.ToArray();
                collection.Clear();
                collection.Add(enumerator.Current);
            }
        }
        if (collection.Count > 0) yield return collection.ToArray();
    }
}