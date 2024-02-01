var file = Lib.FileReader.ReadLines("01");
var instructions = file.Select(s => new Instruction(s)).ToList();

Console.WriteLine($"Part One: {CuboidSize(instructions, true)}");
Console.WriteLine($"Part Two: {CuboidSize(instructions)}");

long CuboidSize(List<Instruction> instructions, bool partOne = false)
{
    var cubes = new List<(bool, Cube)>();
    if (partOne)
    {
        instructions = instructions
            .Where(ins => ins.MinX >= -50 && ins.MaxX <= 50)
            .Where(ins => ins.MinY >= -50 && ins.MaxY <= 50)
            .Where(ins => ins.MinZ >= -50 && ins.MaxZ <= 50)
            .ToList();
    }

    foreach (var instruction in instructions)
    {
        var instructionCube = instruction.Cube;
        var toAdd = new List<(bool, Cube)>();
        if (instruction.IsOn)
        {
            toAdd.Add((true, instructionCube));
        }
        foreach (var (toggle, c) in cubes)
        {
            var intersection = c.Instersection(instructionCube);
            if (intersection is Cube cube)
            {
                toAdd.Add((!toggle, cube));
            }
        }
        cubes.AddRange(toAdd);
    }
    return cubes.Select(t => (t.Item1 ? 1 : -1) * t.Item2.Size)
        .Sum();
}

class Instruction
{
    public bool IsOn { get; init; }
    public int MinX { get; init; }
    public int MaxX { get; init; }
    public int MinY { get; init; }
    public int MaxY { get; init; }
    public int MinZ { get; init; }
    public int MaxZ { get; init; }
    public Cube Cube => new Cube(MinX, MaxX, MinY, MaxY, MinZ, MaxZ);

    public Instruction(string input)
    {
        if (input.StartsWith("on"))
        {
            input = input.Substring(3);
            IsOn = true;
        }
        else
        {
            input = input.Substring(4);
            IsOn = false;
        }
        var split = input.Split(",")
            .Select(s => s.Substring(2))
            .Select(s => s.Split("..").Select(int.Parse).ToList())
            .ToList();

        MinX = split[0][0];
        MaxX = split[0][1];
        MinY = split[1][0];
        MaxY = split[1][1];
        MinZ = split[2][0];
        MaxZ = split[2][1];
    }
}
record Cube(int MinX, int MaxX, int MinY, int MaxY, int MinZ, int MaxZ)
{
    public long Size => ((long)(MaxX - MinX + 1) * (long)(MaxY - MinY + 1) * (long)(MaxZ - MinZ + 1));

    public Cube? Instersection(Cube other)
    {
        if (
            ((this.MaxX <= other.MaxX && this.MaxX >= other.MinX) || (other.MaxX <= this.MaxX && other.MaxX >= this.MinX))
            && ((this.MaxY <= other.MaxY && this.MaxY >= other.MinY) || (other.MaxY <= this.MaxY && other.MaxY >= this.MinY))
            && ((this.MaxZ <= other.MaxZ && this.MaxZ >= other.MinZ) || (other.MaxZ <= this.MaxZ && other.MaxZ >= this.MinZ))
        )
        {
            return new Cube(
                Math.Max(this.MinX, other.MinX),
                Math.Min(this.MaxX, other.MaxX),
                Math.Max(this.MinY, other.MinY),
                Math.Min(this.MaxY, other.MaxY),
                Math.Max(this.MinZ, other.MinZ),
                Math.Min(this.MaxZ, other.MaxZ)
            );
        }
        else
        {
            return null;
        }
    }
}
