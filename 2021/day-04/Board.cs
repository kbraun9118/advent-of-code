public class Board
{

    public Board(string[] lines)
    {
        for (int i = 0; i < size; i++)
        {
            string[] split = lines[i].Split(' ').Where(split => !split.Equals("")).ToArray();
            for (int j = 0; j < size; j++)
            {
                _Board[i, j] = (int.Parse(split[j]), false);
            }
        }
    }

    private const int size = 5;

    private (int, bool)[,] _Board { get; init; } = new (int, bool)[size, size];

    public void CallNumber(int calledNum)
    {
        for (int i = 0; i < _Board.GetLength(0); i++)
        {
            for (int j = 0; j < _Board.GetLength(1); j++)
            {
                var (item, _) = _Board[i, j];
                if (item == calledNum)
                {
                    _Board[i, j] = (calledNum, true);
                    return;
                }
            }
        }
    }

    public bool Bingo()
    {
        //check rows
        for (int i = 0; i < _Board.GetLength(0); i++)
        {
            var isBingo = true;
            for (int j = 0; j < _Board.GetLength(1); j++)
            {
                isBingo = _Board[i, j].Item2 && isBingo;
            }
            if (isBingo)
            {
                return true;
            }
        }

        //check columns
        for (int i = 0; i < _Board.GetLength(0); i++)
        {
            var isBingo = true;
            for (int j = 0; j < _Board.GetLength(1); j++)
            {
                isBingo = _Board[j, i].Item2 && isBingo;
            }
            if (isBingo)
            {
                return true;
            }
        }

        return false;
    }

    public int SumNonCalled()
    {
        return _Board.Cast<(int, bool)>()
                     .Where(tuple => !tuple.Item2)
                     .Select(tuple => tuple.Item1)
                     .Sum();
    }
}