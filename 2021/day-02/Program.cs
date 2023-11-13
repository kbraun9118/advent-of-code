var file = File.ReadAllLines(@"./input.txt");

var endPosition1 = file.Select(line => new Instruction(line))
                      .Aggregate(new Position(0, 0), (acc, instruction) => instruction.UpdatePosition(acc));

Console.WriteLine($"Part One: {endPosition1.Depth * endPosition1.Horizontal}");

var endPosition2 = file.Select(line => new Instruction(line))
                      .Aggregate(new AimPosition(0, 0, 0), (acc, instruction) => instruction.UpdateAimPosition(acc));

Console.WriteLine($"Part Two: {endPosition2.Depth * endPosition2.Horizontal}");