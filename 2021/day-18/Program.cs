class Program
{
    public static void Main(string[] args)
    {
        var file = Lib.FileReader.ReadLines("18").ToArray();

        var snailFishNumber = file
            .Select(s => new SnailfishPair(s))
            .Aggregate((acc, next) => acc + next);

        var magnitude = snailFishNumber.Magnitude();

        var magnitudeMax = file
            .SelectMany((line, i) => file
                .Where((_, j) => i != j)
                .Select((innerLine) => new SnailfishPair(line) + new SnailfishPair(innerLine))
            )
            .Select(f => f.Magnitude())
            .Max();

        Console.WriteLine($"Part One: {magnitude}");
        Console.WriteLine($"Part Two: {magnitudeMax}");
    }
}

public abstract class SnailfishNumber
{
    public abstract bool IsSplitable { get; }
    public abstract SnailfishPair Split(SnailfishPair parent);
    public abstract string AsString();
    public abstract int Magnitude();
}

public class SnailfishValue : SnailfishNumber
{
    public int Value { get; set; }

    public override bool IsSplitable { get => Value > 9; }

    public SnailfishValue(int value)
    {
        Value = value;
    }

    public override SnailfishPair Split(SnailfishPair parent)
    {
        var newSnailfish = new SnailfishPair(new SnailfishValue(Value / 2), new SnailfishValue(Value - Value / 2), parent.Depth + 1);
        newSnailfish.Parent = parent;
        return newSnailfish;
    }

    public override string AsString()
    {
        return Value.ToString();
    }

    public override int Magnitude()
    {
        return Value;
    }
}

public class SnailfishPair : SnailfishNumber
{
    public SnailfishNumber X { get; set; }
    public SnailfishNumber Y { get; set; }
    public int Depth { get; set; }
    public SnailfishPair? Parent { get; set; }
    public bool IsExplodable
    {
        get
        {

            if (Depth > 3) return true;
            var (xExplodable, yExplodable) = (false, false);
            if (X is SnailfishPair x) xExplodable = x.IsExplodable;
            if (Y is SnailfishPair y) yExplodable = y.IsExplodable;

            return xExplodable || yExplodable;
        }
    }
    public override bool IsSplitable { get => X.IsSplitable || Y.IsSplitable; }

    public SnailfishPair(SnailfishNumber x, SnailfishNumber y, int depth)
    {
        X = x;
        Y = y;
        Depth = depth;
    }

    public SnailfishPair(string input)
    {
        var (snailfishNumber, _) = Parse(input);
        if (snailfishNumber is SnailfishPair snailfishPair)
        {
            X = snailfishPair.X;
            Y = snailfishPair.Y;
            Depth = snailfishPair.Depth;
            Parent = null;
            if (snailfishPair.X is SnailfishPair xPair) xPair.Parent = this;
            if (snailfishPair.Y is SnailfishPair yPair) yPair.Parent = this;
        }
        else
        {
            throw new ArgumentException("Input does not parse to SnailfishPair");
        }
    }

    private static (SnailfishNumber snailfishNumber, string Tail) Parse(string input, int depth = 0)
    {
        SnailfishNumber x, y;
        input = input.Substring(1);
        var left = string.Join("", input.TakeWhile(c => Char.IsNumber(c)))!;
        if (left.Length > 0)
        {
            x = new SnailfishValue(int.Parse(left));
            input = input.Substring(left.Length);
        }
        else
        {
            (x, input) = Parse(input, depth + 1);
        }

        input = input.Substring(1);

        var right = string.Join("", input.TakeWhile(c => Char.IsNumber(c)))!;
        if (right.Length > 0)
        {
            y = new SnailfishValue(int.Parse(right));
            input = input.Substring(right.Length);
        }
        else
        {
            (y, input) = Parse(input, depth + 1);
        }
        input = input.Substring(1);
        var newSnailfish = new SnailfishPair(x, y, depth);
        if (x is SnailfishPair xPair) xPair.Parent = newSnailfish;
        if (y is SnailfishPair yPair) yPair.Parent = newSnailfish;
        return (newSnailfish, input);
    }

    public static SnailfishPair operator +(SnailfishPair left, SnailfishPair right)
    {
        left.IncreaseDepth();
        right.IncreaseDepth();
        var newSnailfish = new SnailfishPair(left, right, 0);
        left.Parent = newSnailfish;
        right.Parent = newSnailfish;
        while (newSnailfish.IsExplodable || newSnailfish.IsSplitable)
        {
            if (newSnailfish.IsExplodable) newSnailfish.Explode();
            else newSnailfish.Split();
        }
        return newSnailfish;
    }


    public void IncreaseDepth()
    {
        if (X is SnailfishPair x) x.IncreaseDepth();

        if (Y is SnailfishPair y) y.IncreaseDepth();
        Depth++;
    }

    public void Explode()
    {
        if (IsExplodable && X is SnailfishValue x && Y is SnailfishValue y)
        {
            var current = Parent;
            var previous = this;
            while (current != null && current.X == previous)
            {
                previous = current;
                current = current.Parent;
            }
            if (current != null)
            {
                var left = current.X;
                while (left is SnailfishPair leftCurrent)
                {
                    left = leftCurrent.Y;
                }
                if (left is SnailfishValue leftValue)
                {
                    leftValue.Value += x.Value;
                }
            }
            (current, previous) = (Parent, this);
            while (current != null && current.Y == previous)
            {
                previous = current;
                current = current.Parent;
            }
            if (current != null)
            {
                var right = current.Y;
                while (right is SnailfishPair rightCurrent)
                {
                    right = rightCurrent.X;
                }
                if (right is SnailfishValue rightValue)
                {
                    rightValue.Value += y.Value;
                }
            }
            if (Parent is not null)
            {
                if (this == Parent.X)
                {
                    Parent.X = new SnailfishValue(0);
                }
                else
                {
                    Parent.Y = new SnailfishValue(0);
                }
            }
        }
        else if (IsExplodable && X is SnailfishPair xPair && xPair.IsExplodable) xPair.Explode();
        else if (IsExplodable && Y is SnailfishPair yPair && yPair.IsExplodable) yPair.Explode();
    }

    public override SnailfishPair Split(SnailfishPair? parent = null)
    {
        if (X.IsSplitable)
        {
            var x = X.Split(this);
            X = x;
        }
        else if (Y.IsSplitable)
        {
            var y = Y.Split(this);
            Y = y;
        }
        return this;
    }

    public override string AsString()
    {
        return $"[{X.AsString()},{Y.AsString()}]";
    }

    public override int Magnitude()
    {
        return 3 * X.Magnitude() + 2 * Y.Magnitude();
    }
}
