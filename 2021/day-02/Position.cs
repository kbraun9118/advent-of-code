public class Position
{
    public int Depth { get; init; }
    public int Horizontal { get; init; }

    public Position(int depth, int horizontal)
    {
        Depth = depth;
        Horizontal = horizontal;
    }
}