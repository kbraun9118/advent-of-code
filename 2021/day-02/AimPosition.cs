public class AimPosition
{
    public int Depth { get; init; }
    public int Horizontal { get; init; }
    public int Aim { get; init; }

    public AimPosition(int depth, int horizontal, int aim)
    {
        Depth = depth;
        Horizontal = horizontal;
        Aim = aim;
    }
}