public class SeaFloor
{
    private Octopus[,] Board { get; init; }
    public int Size => Board.Length;

    public SeaFloor(string[] lines)
    {
        Board = new Octopus[lines.Length, lines[0].Length];
        for (int i = 0; i < lines.Length; i++)
        {
            for (int j = 0; j < lines[i].Length; j++)
            {
                Board[i, j] = new Octopus { PowerLevel = int.Parse(lines[i][j].ToString()), HasFlashed = false, Coords = (i, j) };
            }
        }
    }

    public void Print()
    {
        for (int i = 0; i < Board.GetLength(0); i++)
        {
            for (int j = 0; j < Board.GetLength(1); j++)
            {
                Console.Write(Board[i, j].PowerLevel);
            }
            Console.WriteLine("");
        }
    }

    public int Step()
    {
        for (int i = 0; i < Board.GetLength(0); i++)
        {
            for (int j = 0; j < Board.GetLength(1); j++)
            {
                Board[i, j] = Board[i, j].Increment();
            }
        }
        var hasFlashed = Board.Cast<Octopus>().Where(o => o.HasFlashed).ToList();
        while (hasFlashed.Count > 0)
        {
            var cloned = hasFlashed.ToList();
            hasFlashed.Clear();
            foreach (var ocotopus in cloned)
            {
                var adjacents = GetAdjacent(ocotopus).Where(o => !o.HasFlashed);
                foreach (var adjacent in adjacents)
                {
                    var incremented = adjacent.Increment();
                    Board[adjacent.Coords.X, adjacent.Coords.Y] = incremented;
                    if (incremented.HasFlashed)
                    {
                        hasFlashed.Add(incremented);
                    }
                }
            }
        }

        var allFlashed = Board.Cast<Octopus>().Where(o => o.HasFlashed).ToList();
        allFlashed.ForEach(o => Board[o.Coords.X, o.Coords.Y] = o.Reset());
        return allFlashed.Count;
    }

    public IEnumerable<Octopus> GetAdjacent(Octopus octopus)
    {
        var xDiff = new List<int>() { -1, 0, 1 };
        var yDiff = new List<int>() { -1, 0, 1 };
        if (octopus.Coords.X == 0)
        {
            xDiff.Remove(-1);
        }
        if (octopus.Coords.X == Board.GetUpperBound(0))
        {
            xDiff.Remove(1);
        }
        if (octopus.Coords.Y == 0)
        {
            yDiff.Remove(-1);
        }
        if (octopus.Coords.Y == Board.GetUpperBound(1))
        {
            yDiff.Remove(1);
        }

        return from x in xDiff
               from y in yDiff
               where (x, y) != (0, 0)
               select Board[octopus.Coords.X + x, octopus.Coords.Y + y];
    }
}

public record Octopus
{
    public int PowerLevel { get; init; }
    public bool HasFlashed { get; init; }
    public (int X, int Y) Coords { get; init; }

    public Octopus Increment()
    {
        if (PowerLevel > 8)
        {
            return this with { PowerLevel = 0, HasFlashed = true };
        }
        else
        {
            return this with { PowerLevel = PowerLevel + 1 };
        }
    }

    public Octopus Reset()
    {
        return this with { HasFlashed = false };
    }
}
