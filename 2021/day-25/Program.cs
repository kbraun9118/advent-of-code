using System;
using System.IO;

var file = File.ReadAllLines(@"./input.txt");
var seaFloor = new SeaFloor(file);
var count = 0;

while (true)
{
    var next = seaFloor.Move();
    count++;
    if (next == seaFloor)
    {
        break;
    }
    else
    {
        seaFloor = next;
    }
}

Console.WriteLine($"Part One: {count}");

record Point(int X, int Y);
enum Direction
{
    R, D
}

class SeaFloor
{
    public Dictionary<Point, Direction> Floor { get; init; } = new();
    public int MaxX { get; init; }
    public int MaxY { get; init; }

    public SeaFloor(string[] input)
    {
        MaxX = input[0].Length;
        MaxY = input.Length;
        for (int i = 0; i < input.Length; i++)
        {
            for (int j = 0; j < input[i].Length; j++)
            {
                if (input[i][j] == '>') Floor.Add(new Point(j, i), Direction.R);
                else if (input[i][j] == 'v') Floor.Add(new Point(j, i), Direction.D);
            }
        }
    }

    public SeaFloor() { }

    public SeaFloor Move()
    {
        var afterRight = new SeaFloor { MaxX = MaxX, MaxY = MaxY };

        foreach (var floor in Floor)
        {
            var next = floor.Key with { X = floor.Key.X + 1 == MaxX ? 0 : floor.Key.X + 1 };

            if (floor.Value == Direction.R && Floor.ContainsKey(next) is false)
            {
                afterRight.Floor.Add(next, Direction.R);
            }
            else
            {
                afterRight.Floor.Add(floor.Key, floor.Value);
            }
        }

        var afterDown = new SeaFloor { MaxX = MaxX, MaxY = MaxY };

        foreach (var floor in afterRight.Floor)
        {
            var next = floor.Key with { Y = floor.Key.Y + 1 == MaxY ? 0 : floor.Key.Y + 1 };

            if (floor.Value == Direction.D && afterRight.Floor.ContainsKey(next) is false)
            {
                afterDown.Floor.Add(next, Direction.D);
            }
            else
            {
                afterDown.Floor.Add(floor.Key, floor.Value);
            }
        }

        return afterDown;
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(Floor);
    }

    public override bool Equals(object? obj)
    {
        return obj is SeaFloor floor &&
               Floor.Count == floor.Floor.Count &&
               Floor.All(pair => floor.Floor.ContainsKey(pair.Key) && floor.Floor[pair.Key] == pair.Value);
    }

    public static bool operator ==(SeaFloor left, SeaFloor? right)
    {
        return left.Equals(right);
    }

    public static bool operator !=(SeaFloor left, SeaFloor? right)
    {
        return !(left == right);
    }
}