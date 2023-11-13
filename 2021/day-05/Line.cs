public class Line
{
    public Point Start { get; set; }
    public Point End { get; set; }

    public Line(Point start, Point end)
    {
        Start = start;
        End = end;
    }

    public Line(string line)
    {
        var split = line.Split(" -> ");
        Start = new Point(split[0]);
        End = new Point(split[1]);
    }

    public int MaxX()
    {
        return Start.X > End.X ? Start.X : End.X;
    }

    public int MaxY()
    {
        return Start.Y > End.Y ? Start.Y : End.Y;
    }


    public bool IsHorizontal()
    {
        return Start.Y == End.Y;
    }

    public bool IsVertical()
    {
        return Start.X == End.X;
    }

    public IEnumerable<Point> PointsBetween()
    {
        yield return Start;

        var current = Start;
        var (xOffset, yOffset) = (0, 0);
        if (IsHorizontal())
        {
            if (Start.X > End.X)
            {
                xOffset = -1;
            }
            else
            {
                xOffset = 1;
            }
        }
        else if (IsVertical())
        {
            if (Start.Y > End.Y)
            {
                yOffset = -1;
            }
            else
            {
                yOffset = 1;
            }
        }
        else if (Start.X > End.X)
        {
            if (Start.Y > End.Y)
            {
                xOffset = -1;
                yOffset = -1;
            }
            else
            {
                xOffset = -1;
                yOffset = 1;
            }
        }
        else if (Start.Y > End.Y)
        {
            xOffset = 1;
            yOffset = -1;
        }
        else
        {
            xOffset = 1;
            yOffset = 1;
        }


        while (!current.Equals(End))
        {
            current = new Point(current.X + xOffset, current.Y + yOffset);
            yield return current;
        }
    }
}