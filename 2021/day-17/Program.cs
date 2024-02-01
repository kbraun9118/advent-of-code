public class Program
{
    public static void Main(string[] args)
    {
        var file = Lib.FileReader.ReadLines("17")[0];

        var ranges = file.Substring(15).Split(", ");
        var xRanges = ranges[0].Split("..").Select(s => int.Parse(s)).ToArray();
        var yRanges = ranges[1].Substring(2).Split("..").Select(s => int.Parse(s)).ToArray();

        var xRange = Enumerable.Range(xRanges[0], xRanges[1] - xRanges[0] + 1);
        var yRange = Enumerable.Range(yRanges[0], yRanges[1] - yRanges[0] + 1);

        var yMaxs = MaxY(xRange, yRange);

        Console.WriteLine($"Part One: {yMaxs.MaxBy(val => val.Item2).Item2}");
        Console.WriteLine($"Part Two: {yMaxs.Count()}");
    }

    public static List<((int, int), int)> MaxY(IEnumerable<int> xRange, IEnumerable<int> yRange)
    {
        var yMaxs = new List<((int, int), int)>();
        for (int xStarting = 0; xStarting < xRange.Max() + 1; xStarting++)
        {
            for (int yStarting = yRange.Min(); yStarting < yRange.Select(Math.Abs).Max() + 1; yStarting++)
            {
                var (xVelocity, yVelocity) = (xStarting, yStarting);
                (int X, int Y) position = (0, 0);
                var yMaxForVel = int.MinValue;
                var hitTarget = false;
                while (position.X < xRange.Max() && position.Y > yRange.Min())
                {
                    position = (position.X + xVelocity, position.Y + yVelocity);
                    if (xVelocity > 0) xVelocity--;
                    yVelocity--;
                    if (position.Y > yMaxForVel) yMaxForVel = position.Y;
                    if (xRange.Contains(position.X) && yRange.Contains(position.Y)) hitTarget = true;
                }
                if (hitTarget) yMaxs.Add(((xStarting, yStarting), yMaxForVel));
            }
        }
        return yMaxs;
    }
}
