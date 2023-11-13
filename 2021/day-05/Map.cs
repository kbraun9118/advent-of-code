public class Map
{
    public int[,] Board;

    public Map(int x, int y)
    {
        Board = new int[x, y];
    }

    public void AddToMap(IEnumerable<Point> points)
    {
        foreach (var point in points)
        {
            Board[point.X, point.Y] += 1;
        }
    }

    public int Overlap()
    {
        return Board.Cast<int>()
                    .Where(point => point > 1)
                    .Count();
    }

    public void PrintBoard()
    {
        for (int i = 0; i < Board.GetLength(0); i++)
        {
            for (int j = 0; j < Board.GetLength(1); j++)
            {
                Console.Write($"{Board[i, j]}");
            }
            Console.Write("\n");
        }
    }

    public void Clear()
    {
        for (int i = 0; i < Board.GetLength(0); i++)
        {
            for (int j = 0; j < Board.GetLength(1); j++)
            {
                Board[i, j] = 0;
            }
        }
    }
}