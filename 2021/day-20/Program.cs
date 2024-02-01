using Lib;

var file = Lib.FileReader.ReadLines("20");

var key = file[0];
var trench = new Trench(file[2..]);

trench = trench.Enhance(key);
trench = trench.Enhance(key);

Console.WriteLine($"Part One: {trench.Points.Count}");

for (int i = 0; i < 48; i++)
{
    trench = trench.Enhance(key);
}

Console.WriteLine($"Part Two: {trench.Points.Count}");

public class Trench
{
    public HashSet<(int X, int Y)> Points { get; init; } = new();
    public char Fill { get; init; } = '.';
    public int MinY { get; init; }
    public int MaxY { get; init; }
    public int MinX { get; init; }
    public int MaxX { get; init; }

    public char this[(int X, int Y) point]
    {
        get
        {
            if (point.X >= MaxX || point.X < MinX || point.Y >= MaxY || point.Y < MinY) return Fill;
            return Points.Contains(point) ? '#' : '.';
        }
    }

    public Trench(HashSet<(int X, int Y)> points, char fill, int minY, int maxY, int minX, int maxX)
    {
        Points = points;
        Fill = fill;
        MinY = minY;
        MaxY = maxY;
        MinX = minX;
        MaxX = maxX;
    }

    public Trench(IEnumerable<string> strings)
    {
        MinX = 0;
        MinY = 0;
        MaxY = strings.Count();
        MaxX = strings.ToList()[0].Length;
        foreach (var (str, i) in strings.WithIndex())
        {
            foreach (var (c, j) in str.WithIndex())
            {
                if (c == '#')
                {
                    Points.Add((j, i));
                }
            }
        }
    }

    public int SearchIndex((int X, int Y) point)
    {
        var diffs = from yDiff in Enumerable.Range(-1, 3)
                    from xDiff in Enumerable.Range(-1, 3)
                    select (xDiff, yDiff);

        return Convert.ToInt32(
            string.Join("", diffs.Select(d => this[(point.X + d.xDiff, point.Y + d.yDiff)]).Select(c => c == '#' ? 1 : 0)),
            2
        );
    }

    public void PrintTrench()
    {
        for (int y = MinY; y < MaxY; y++)
        {
            for (int x = MinX; x < MaxX; x++)
            {
                Console.Write(this[(x, y)]);
            }
            Console.Write('\n');
        }
    }

    public Trench Enhance(string key)
    {
        var points = new HashSet<(int X, int Y)>();
        for (int y = MinY - 1; y < MaxY + 1; y++)
        {
            for (int x = MinX - 1; x < MaxX + 1; x++)
            {
                if (key[SearchIndex((x, y))] == '#')
                {
                    points.Add((x, y));
                }
            }
        }
        return new Trench(points, key[SearchIndex((int.MinValue, int.MinValue))], MinY - 1, MaxY + 1, MinX - 1, MaxX + 1);
    }
}

