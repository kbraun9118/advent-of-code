public class Instruction
{

    public enum SubDirection
    {
        Forward,
        Down,
        Up
    }

    public SubDirection Direction { get; init; }
    public int Amount { get; init; }

    public Instruction(string input)
    {
        var split = input.Split(' ');
        Direction = split[0] switch
        {
            "forward" => SubDirection.Forward,
            "down" => SubDirection.Down,
            "up" => SubDirection.Up,
            _ => throw new ArgumentException("Direction must be either 'forward', 'up', or 'down'.")
        };
        Amount = int.Parse(split[1]);
    }

    public Position UpdatePosition(Position position)
    {
        return Direction switch
        {
            SubDirection.Forward => new Position(position.Depth, position.Horizontal + Amount),
            SubDirection.Down => new Position(position.Depth + Amount, position.Horizontal),
            SubDirection.Up => new Position(position.Depth - Amount, position.Horizontal),
            _ => throw new ArgumentException("Invalid Direction")
        };
    }

    public AimPosition UpdateAimPosition(AimPosition aimPosition)
    {
        return Direction switch
        {
            SubDirection.Forward => new AimPosition(aimPosition.Depth + (Amount * aimPosition.Aim), aimPosition.Horizontal + Amount, aimPosition.Aim),
            SubDirection.Down => new AimPosition(aimPosition.Depth, aimPosition.Horizontal, aimPosition.Aim + Amount),
            SubDirection.Up => new AimPosition(aimPosition.Depth, aimPosition.Horizontal, aimPosition.Aim - Amount),
            _ => throw new ArgumentException("Invalid Direction")
        };
    }
}