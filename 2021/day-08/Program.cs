var file = Lib.FileReader.ReadLines("08");

var partOne = file.SelectMany(line => line.Split(" | ")[1].Split(" ").Where(digit => digit.Length == 2 || digit.Length == 4 || digit.Length == 3 || digit.Length == 7))
    .Count();

Console.WriteLine($"Part One: {partOne}");

var decoder = (string[] decoded) =>
{
    var dict = new Dictionary<string, string>();
    var decodedList = new List<string>(decoded);

    var eightKey = decodedList.Find(key => key.Count() == 7)!;
    decodedList.Remove(eightKey);
    dict[eightKey] = "8";

    var oneKey = decodedList.Find(key => key.Count() == 2)!;
    decodedList.Remove(oneKey);
    dict[oneKey] = "1";

    var fourKey = decodedList.Find(key => key.Count() == 4)!;
    decodedList.Remove(fourKey);
    dict[fourKey] = "4";

    var sevenKey = decodedList.Find(key => key.Count() == 3)!;
    decodedList.Remove(sevenKey);
    dict[sevenKey] = "7";

    var lengthFiveKeys = decodedList.Where(key => key.Count() == 5).ToList();

    var threeKey = lengthFiveKeys.Find(key => key.ContainsAllChars(oneKey))!;
    decodedList.Remove(threeKey);
    lengthFiveKeys.Remove(threeKey);
    dict[threeKey] = "3";

    var fiveKey = lengthFiveKeys.Find(key => key.ContainsAllChars(fourKey.Difference(oneKey)))!;
    decodedList.Remove(fiveKey);
    lengthFiveKeys.Remove(fiveKey);
    dict[fiveKey] = "5";

    var twoKey = lengthFiveKeys[0];
    decodedList.Remove(twoKey);
    dict[twoKey] = "2";

    var sixKey = decodedList.Find(key => !key.ContainsAllChars(sevenKey))!;
    decodedList.Remove(sixKey);
    dict[sixKey] = "6";

    var nineKey = decodedList.Find(key => key.ContainsAllChars(fourKey))!;
    decodedList.Remove(nineKey);
    dict[nineKey] = "9";
    dict[decodedList[0]] = "0";

    return dict;
};

var partTwo = file.Select(line =>
{
    var split = line.Split(" | ");
    var dict = decoder(split[0].Split(" "));

    return int.Parse(String.Concat(split[1].Split(" ").Select(encoded => dict.First(tuple => tuple.Key.ContainsExactly(encoded)).Value)));
}).Sum();

Console.WriteLine($"Part Two: {partTwo}");
