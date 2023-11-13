var file = File.ReadAllLines(@"./input.txt");

var called = file[0];

var boards = file.Skip(2)
                 .Chunk(6)
                 .Select(chunks => new Board(chunks.Take(5).ToArray()))
                 .ToList();

var lastCalled = 0;

var calledArr = called.Split(',').Select(int.Parse).ToArray();

foreach (var calledNum in calledArr)
{
    foreach (var board in boards)
    {
        board.CallNumber(calledNum);
    }

    if (boards.Any(board => board.Bingo()))
    {
        lastCalled = calledNum;
        break;
    }
}

var part1 = boards.Find(board => board.Bingo())?.SumNonCalled() ?? 0;

Console.WriteLine($"Part One: {part1 * lastCalled}");

foreach (var calledNum in calledArr)
{
    if (boards.Count() == 1 && boards[0].Bingo())
    {
        break;
    }

    var boardsToRemove = new List<Board>();

    foreach (var board in boards)
    {
        board.CallNumber(calledNum);
        if (board.Bingo() && boards.Count() > 1)
        {
            boardsToRemove.Add(board);
        }

        if (boards.Count() == 1 && board.Bingo())
        {
            lastCalled = calledNum;
        }
    }

    foreach (var boardToRemove in boardsToRemove)
    {
        boards.Remove(boardToRemove);
    }
}

var part2 = boards[0].SumNonCalled() * lastCalled;

Console.WriteLine($"Part Two: {part2}");
