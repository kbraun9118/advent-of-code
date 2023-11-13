var file = File.ReadAllLines(@"./input.txt")[0];

var generation = new long[9];

foreach (var line in file.Split(','))
{
    generation[int.Parse(line)] += 1;
}

var runSimulation = (long[] prevGeneragtion) =>
{
    var nextGeneration = new long[9];

    nextGeneration[8] = prevGeneragtion[0];
    nextGeneration[6] = prevGeneragtion[7] + prevGeneragtion[0];
    for (int i = 0; i < 8; i++)
    {
        if (i != 6) nextGeneration[i] = prevGeneragtion[i + 1];
    }
    return nextGeneration;
};

for (int i = 0; i < 80; i++)
{
    generation = runSimulation(generation);
}

Console.WriteLine($"Day One: {generation.Sum()}");


for (int i = 0; i < 256 - 80; i++)
{
    generation = runSimulation(generation);
}

Console.WriteLine($"Day Two: {generation.Sum()}");