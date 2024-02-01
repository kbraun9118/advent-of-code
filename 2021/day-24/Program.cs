using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text;

Console.WriteLine($"Part One: {PartOne()}");
Console.WriteLine($"Part Two: {PartTwo()}");

static (long result, long time) PartOne()
{
    var sw = new Stopwatch();
    sw.Start();
    var lines = Lib.FileReader.ReadLines("24");
    var pairs = new List<(int, int)>();
    foreach (var i in Enumerable.Range(0, 14))
    {
        pairs.Add((int.Parse(lines[i * 18 + 5][6..]), int.Parse(lines[i * 18 + 15][6..])));
    }
    var stack = new Stack<(int, int)>();
    var keys = new Dictionary<int, (int, int)>();

    foreach (var i in pairs.Select((pair, i) => (pair, i)))
    {
        if (i.pair.Item1 > 0)
        {
            stack.Push((i.i, i.pair.Item2));
        }
        else
        {
            var (j, jj) = stack.Pop();
            keys[i.i] = (j, jj + i.pair.Item1);
        }
    }
    var output = new Dictionary<int, int>();

    foreach (var kvp in keys)
    {
        output[kvp.Key] = Math.Min(9, 9 + kvp.Value.Item2);
        output[kvp.Value.Item1] = Math.Min(9, 9 - kvp.Value.Item2);
    }
    var result = long.Parse(string.Join("", output.OrderBy(x => x.Key).Select(x => x.Value)));
    sw.Stop();

    return (result, sw.ElapsedMilliseconds);
}

static (long result, long time) PartTwo()
{
    var sw = new Stopwatch();
    sw.Start();
    var lines = Lib.FileReader.ReadLines("01");
    var pairs = new List<(int, int)>();
    foreach (var i in Enumerable.Range(0, 14))
    {
        pairs.Add((int.Parse(lines[i * 18 + 5][6..]), int.Parse(lines[i * 18 + 15][6..])));
    }
    var stack = new Stack<(int, int)>();
    var keys = new Dictionary<int, (int, int)>();

    foreach (var (pair, i) in pairs.Select((pair, i) => (pair, i)))
    {
        if (pair.Item1 > 0)
        {
            stack.Push((i, pair.Item2));
        }
        else
        {
            var (j, addr) = stack.Pop();
            keys[i] = (j, addr + pair.Item1);
        }
    }
    var output = new Dictionary<int, int>();

    foreach (var kvp in keys)
    {
        output[kvp.Key] = Math.Max(1, 1 + kvp.Value.Item2);
        output[kvp.Value.Item1] = Math.Max(1, 1 - kvp.Value.Item2);
    }
    var result = long.Parse(String.Join("", output.OrderBy(x => x.Key).Select(x => x.Value)));
    sw.Stop();

    return (result, sw.ElapsedMilliseconds);
}

