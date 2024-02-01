public class Program
{
    public static void Main(string[] args)
    {
        var file = Lib.FileReader.ReadLines("15");

        var cave = new Cave(file);
        var partOne = cave.ShortestPath();
        Console.WriteLine($"Part One: {partOne}");

        var cave2 = cave.IncreaseDimensions(5);
        var partTwo = cave2.ShortestPath();
        Console.WriteLine($"Part Two: {partTwo}");
    }
}

public class Node
{
    public (int X, int Y) Position { get; init; }
    public int Risk { get; init; }
    public bool Visited { get; set; } = false;
    public int Dist { get; set; } = int.MaxValue;

    public override bool Equals(object? obj)
    {
        return obj is Node node &&
               Position.Equals(node.Position);
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(Position);
    }

    public static bool operator ==(Node left, Node right)
    {
        return left.Position == right.Position;
    }

    public static bool operator !=(Node left, Node right)
    {
        return !(left == right);
    }


}

public class Cave
{
    private Node[,] Floor { get; init; }

    public Cave(string[] lines)
    {
        Floor = new Node[lines.Length, lines[0].Length];

        for (int i = 0; i < lines.Length; i++)
        {
            for (int j = 0; j < lines[0].Length; j++)
            {
                Floor[i, j] = new()
                {
                    Risk = int.Parse(lines[i][j].ToString()),
                    Position = (i, j)
                };
            }
        }
    }

    private Cave(int x, int y)
    {
        Floor = new Node[x, y];
    }

    public Cave IncreaseDimensions(int increase)
    {
        var next = new Cave(Floor.GetLength(0) * increase, Floor.GetLength(1) * increase);

        for (int i = 0; i < next.Floor.GetLength(0); i++)
        {
            for (int j = 0; j < next.Floor.GetLength(1); j++)
            {
                var value = (Floor[i % Floor.GetLength(0), j % Floor.GetLength(1)].Risk) + (i / Floor.GetLength(0)) + (j / Floor.GetLength(1));
                next.Floor[i, j] = new()
                {
                    Risk = value > 9 ? value % 10 + 1 : value,
                    Position = (i, j)
                };
            }
        }

        return next;
    }

    public List<Node> GetNeighbors(Node node)
    {
        return new List<(int X, int Y)>
        {
            (1,0),
            (-1,0),
            (0,-1),
            (0,1),
        }.Select(e => (node.Position.X + e.X, node.Position.Y + e.Y))
        .Where(p => p.Item1 >= 0 && p.Item1 <= Floor.GetUpperBound(0))
        .Where(p => p.Item2 >= 0 && p.Item2 <= Floor.GetUpperBound(1))
        .Select(p => Floor[p.Item1, p.Item2])
        .Where(p => !p.Visited)
        .ToList();
    }

    public int ShortestPath()
    {
        Floor[0, 0].Dist = 0;
        var unvisited = Floor.Cast<Node>().ToList();

        while (unvisited.Count > 0)
        {
            var current = unvisited.MinBy(n => n.Dist)!;
            unvisited.Remove(current);
            current.Visited = true;
            foreach (var neighbor in GetNeighbors(current))
            {
                if (neighbor.Dist > current.Dist + neighbor.Risk)
                {
                    neighbor.Dist = current.Dist + neighbor.Risk;
                }
            }
        }

        return Floor[Floor.GetUpperBound(0), Floor.GetUpperBound(1)].Dist;
    }

    public void PrintFloor()
    {
        for (int j = 0; j < Floor.GetLength(0); j++)
        {
            for (int i = 0; i < Floor.GetLength(1); i++)
            {
                Console.Write(Floor[j, i].Risk);
            }
            Console.WriteLine();
        }
    }
}
