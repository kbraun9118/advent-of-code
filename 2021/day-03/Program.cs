var file = Lib.FileReader.ReadLines("03");
// Convert.ToInt32(file.First(), 2)

var createRates = (string[] lines) =>
{
    var numLength = lines[0].Length;
    var fileLength = lines.Length;

    var averageColumn = lines.Select(line => line.ToCharArray()).Aggregate(
        new int[numLength],
        (acc, line) => acc.Zip(line).Select(tuple =>
            {
                if (tuple.Second.Equals('1'))
                {
                    return tuple.First + 1;
                }
                else
                {
                    return tuple.First;
                }
            }).ToArray());

    var gamma = averageColumn.Select(num =>
        {
            if (num >= fileLength - num)
            {
                return '1';
            }
            else
            {
                return '0';
            }
        }).ToArray();

    var epsilon = averageColumn.Select(num =>
        {
            if (num < fileLength - num)
            {
                return '1';
            }
            else
            {
                return '0';
            }
        }).ToArray();
    return (gamma, epsilon);
};

var (gamma, epsilon) = createRates(file);

var gammaValue = Convert.ToInt64(new string(gamma), 2);
var epsilonValue = Convert.ToInt64(new string(epsilon), 2);

Console.WriteLine($"Part One: {gammaValue * epsilonValue}");

var oxygen = file.Select(line => line).ToList();
var oxygenRate = gamma;

for (int i = 0; oxygen.Count() > 1; i++)
{
    oxygen = oxygen.Where(ox => ox[i].Equals(oxygenRate[i])).ToList();
    if (oxygen.Count() > 1)
    {
        var tuple = createRates(oxygen.ToArray());
        oxygenRate = tuple.gamma;
    }
}

var carbon = file.Select(line => line).ToList();
var carbonRate = epsilon;

for (int i = 0; carbon.Count() > 1; i++)
{
    carbon = carbon.Where(co => co[i].Equals(carbonRate[i])).ToList();
    if (carbon.Count() > 1)
    {
        var tuple = createRates(carbon.ToArray());
        carbonRate = tuple.epsilon;
    }
}

var oxygenValue = Convert.ToInt64(new string(oxygen[0]), 2);
var carbonValue = Convert.ToInt64(new string(carbon[0]), 2);

Console.WriteLine($"Part Two: {oxygenValue * carbonValue}");
