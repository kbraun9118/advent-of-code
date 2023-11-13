public class SeaFloor
{
    public SeaFloor(string[] input)
    {
        var splitInput = input.Select(row => row.ToCharArray()).ToArray();
        Floor = new int[splitInput.Length, splitInput[0].Length];
        for (var i = 0; i < splitInput.Length; i++)
        {
            for (int j = 0; j < splitInput[0].Length; j++)
            {
                Floor[i, j] = int.Parse(splitInput[i][j].ToString());
            }
        }
    }

    private int[,] Floor { get; init; }

    public int Depth(Point point)
    {
        return Depth(point.X, point.Y);
    }

    public int Depth(int x, int y)
    {
        return Floor[x, y];
    }

    public Point[] LowPoints()
    {
        var points = new List<Point>();
        for (int i = 0; i < Floor.GetLength(0); i++)
        {
            for (int j = 0; j < Floor.GetLength(1); j++)
            {
                var adjacent = Adjacent(i, j);
                if (adjacent.All(item => Floor[item.X, item.Y] > Floor[i, j]))
                {
                    points.Add(Point.of(i, j));
                }
            }
        }
        return points.ToArray();
    }

    public Point[] Adjacent(Point point)
    {
        return Adjacent(point.X, point.Y);
    }

    public Point[] Adjacent(int x, int y)
    {
        var xDiffs = new List<int>() { 1, -1 };
        var yDiffs = new List<int>() { 1, -1 };
        if (x == 0)
        {
            xDiffs.Remove(-1);
        }
        if (x == Floor.GetUpperBound(0))
        {
            xDiffs.Remove(1);
        }
        if (y == 0)
        {
            yDiffs.Remove(-1);
        }
        if (y == Floor.GetUpperBound(1))
        {
            yDiffs.Remove(1);
        }

        return xDiffs.Select(xDiff => Point.of(x + xDiff, y)).Concat(
            yDiffs.Select(yDiff => Point.of(x, y + yDiff))
        ).ToArray();
    }

    public int BasinSize(Point point)
    {
        var adjacents = Adjacent(point).Where(adjacent => Depth(adjacent) < 9).ToList();
        var basin = new HashSet<Point>();
        basin.Add(point);

        while (adjacents.Count > 0)
        {
            var clonedAdjacents = new List<Point>(adjacents).AsEnumerable();
            adjacents.ForEach(adjacent => basin.Add(adjacent));
            adjacents.Clear();

            foreach (var adjacent in clonedAdjacents)
            {
                IEnumerable<Point> adjacentsToAdd = Adjacent(adjacent).Where(next => Depth(next) < 9).Where(next => !basin.Contains(next));
                adjacents.AddRange(adjacentsToAdd);
            }
        }

        return basin.Count;
    }
}

public record Point
{
    public int X { get; init; }
    public int Y { get; init; }

    private Point(int x, int y)
    {
        X = x;
        Y = y;
    }

    public static Point of(int x, int y)
    {
        return new Point(x, y);
    }
}