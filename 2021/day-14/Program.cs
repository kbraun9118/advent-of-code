var file = File.ReadAllLines(@"./input.txt").ToList();

var template = file[0];
var pairs = file.Skip(2).Select(s => s.Split(" -> ")).ToList();

var key = pairs.ToDictionary(s => s[0], s => new List<string> { s[0][0] + s[1], s[1] + s[0][1] });
var buckets = pairs.Aggregate(new Dictionary<string, long>(), (acc, next) =>
{
    acc[next[0]] = 0;
    return acc;
});


var filledBuckets = template
    .ToCharArray()
    .Zip(template.ToCharArray().Skip(1))
    .SelectMany(z => key[z.First.ToString() + z.Second])
    .Aggregate(new Dictionary<string, long>(buckets), (acc, next) =>
    {
        acc[next]++;
        return acc;
    });

var expand = (Dictionary<string, long> input) =>
    input
        .SelectMany(p => key[p.Key].Select(v => (v, p.Value)))
        .Aggregate(new Dictionary<string, long>(buckets), (acc, next) =>
        {
            acc[next.v] += next.Value;
            return acc;
        });

var charCount = (Dictionary<string, long> input) =>
    input
        .SelectMany(kv => kv.Key.ToCharArray().Select(c => (c, kv.Value)))
        .Aggregate(new Dictionary<char, long>(), (acc, next) =>
        {
            if (acc.ContainsKey(next.c))
            {
                acc[next.c] += next.Value;
            }
            else
            {
                acc[next.c] = next.Value;
            }
            return acc;
        });

for (long i = 0; i < 9; i++)
{
    filledBuckets = expand(filledBuckets);
}

var dict = charCount(filledBuckets);

var partOneMax = dict.MaxBy(pair => pair.Value).Value;
var partOneMin = dict.MinBy(pair => pair.Value).Value;

var partOne = (partOneMax % 2 == 0 ? partOneMax : partOneMax + 1) / 2 - (partOneMin % 2 == 0 ? partOneMin : partOneMin + 2) / 2;

Console.WriteLine($"Part One: {partOne}");

for (long i = 0; i < 30; i++)
{
    filledBuckets = expand(filledBuckets);
}

var dictTwo = charCount(filledBuckets);

var partTwoMax = dictTwo.MaxBy(pair => pair.Value).Value;
var partTwoMin = dictTwo.MinBy(pair => pair.Value).Value;

var partTwo = (partTwoMax % 2 == 0 ? partTwoMax : partTwoMax + 1) / 2 - (partTwoMin % 2 == 0 ? partTwoMin : partTwoMin + 2) / 2;

Console.WriteLine($"Part Two: {partTwo}");
