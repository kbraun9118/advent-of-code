namespace Lib
{

    public static class Extensions
    {
        public static IEnumerable<(TSource Item, int Index)> WithIndex<TSource>(this IEnumerable<TSource> enumerable)
        {
            var index = 0;
            using var enumerator = enumerable.GetEnumerator();

            while (enumerator.MoveNext())
            {
                yield return (enumerator.Current, index);
                index++;
            }
        }


    }

    public static class FileReader
    {
        public static string[] ReadLines(string day)
        {
            return File.ReadAllLines(@$"../input/2021/{day}/input.txt");
        }
    }
}
