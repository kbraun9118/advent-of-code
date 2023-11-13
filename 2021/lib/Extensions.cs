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
}