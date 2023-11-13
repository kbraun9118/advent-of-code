var file = File.ReadAllLines(@"./input.txt");

var seaFloor = new SeaFloor(file);
var partOne = 0;
var partTwo = -1;

for (int i = 0; i < 100; i++)
{
    var flashed = seaFloor.Step();
    partOne += flashed;
    if (flashed == seaFloor.Size) {
        partTwo = i + 1;
    }
}
var counter = 100;
while (partTwo == -1)
{
    counter++;
    var flashed = seaFloor.Step();
    if (flashed == seaFloor.Size)
    {
        partTwo = counter;
    }
}


Console.WriteLine($"Part One: {partOne}");
Console.WriteLine($"Part Two: {partTwo}");