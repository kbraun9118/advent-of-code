public static class Extensions
{
    public static string Difference(this string left, string right)
    {
        return String.Concat(left.Where(character => !right.Contains(character)) ?? "");
    }

    public static bool ContainsAllChars(this string left, string right)
    {
        return right.All(character => left.Contains(character));
    }

    public static bool ContainsExactly(this string left, string right)
    {
        return left.Length == right.Length && left.ContainsAllChars(right);
    }
}