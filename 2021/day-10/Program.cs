public class Program
{

    public static (char?, string?) findFirstError(string line)
    {
        var rest = line;
        char? error = null;
        string? unpaired = null;

        while (error is null && rest.Length > 0)
        {
            (error, rest, _, unpaired) = parse(rest);
        }
        return (error, unpaired);
    }

    public static (char?, string, bool, string?) parse(string line)
    {
        var first = ' ';
        try
        {
            if (
                line[0] == '(' && line[1] == ')'
                || line[0] == '[' && line[1] == ']'
                || line[0] == '{' && line[1] == '}'
                || line[0] == '<' && line[1] == '>'
            ) return (null, line.Substring(2), false, null);

            first = line[0];
            var (error, rest, isOOB, unPaired) = parse(line.Substring(1));

            if (isOOB)
            {
                return (null, "", true, unPaired + first);
            }

            while (error is null && (rest[0] == '(' || rest[0] == '{' || rest[0] == '<' || rest[0] == '['))
            {
                (error, rest, isOOB, unPaired) = parse(rest);
                if (isOOB)
                {
                    return (null, "", true, unPaired + first);
                }
            }

            if (error is not null) return (error, rest, false, null);

            if (
                first == '(' && rest[0] == ')'
                || first == '[' && rest[0] == ']'
                || first == '{' && rest[0] == '}'
                || first == '<' && rest[0] == '>'
            ) return (null, rest.Substring(1), false, null);

            return (rest[0], rest.Substring(1), false, null);
        }
        catch (IndexOutOfRangeException)
        {
            return (null, "", true, first.ToString().Trim() + (line.Length == 1 ? line : ""));
        }
    }

    public static void Main(string[] args)
    {
        var file = Lib.FileReader.ReadLines("10");

        var errors = file
            .Select(line => findFirstError(line))
            .ToList();

        var partOne = errors
            .Where(tuple => tuple.Item1 is not null)
            .Select(tuple =>
            {
                var error = tuple.Item1;
                if (error == ')') return 3;
                if (error == ']') return 57;
                if (error == '}') return 1197;
                else return 25137;
            }).Sum();

        Console.WriteLine($"Part One: {partOne}");

        var partTwo = errors
            .Where(tuple => tuple.Item1 is null && tuple.Item2 is not null)
            .Select(tuple => tuple.Item2!
                .ToCharArray()
                .Select(item => item switch
                {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    _ => 4
                }).Aggregate(0L, (acc, next) => acc * 5 + next)
            ).OrderBy(item => item)
            .ToList();

        Console.WriteLine($"Part Two: {partTwo[partTwo.Count / 2]}");
    }
}
