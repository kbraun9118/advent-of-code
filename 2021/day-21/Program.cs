var file = Lib.FileReader.ReadLines("21");
var player1Start = int.Parse(file[0].Substring(28));
var player2Start = int.Parse(file[1].Substring(28));
var player1 = new Player { Position = player1Start };
var player2 = new Player { Position = player2Start };
var die = new Die();

while (player1.Score < 1000 && player2.Score < 1000)
{
    player1.Move(die.Roll() + die.Roll() + die.Roll());
    if (player1.Score >= 1000) break;
    player2.Move(die.Roll() + die.Roll() + die.Roll());
}

var loserScore = Math.Min(player1.Score, player2.Score);

Console.WriteLine($"Part One: {loserScore * die.Rolls}");

var winTracker = new WinTracker();

var possibleRolls = (from x in Enumerable.Range(1, 3)
                     from y in Enumerable.Range(1, 3)
                     from z in Enumerable.Range(1, 3)
                     select x + y + z).ToList();
var possibilityDict = possibleRolls
    .GroupBy(n => n)
    .ToDictionary(n => n.Key, n => (ulong)n.Count());

RollForPlayer(0, 0, player1Start, player2Start, true, 1, winTracker);

Console.WriteLine($"Part Two: {Math.Max(winTracker.PlyaerOneWins, winTracker.PlyaerTwoWins)}");

void RollForPlayer(int playerOneScore, int playerTwoScore, int playerOnePosition, int playerTwoPosition, bool playerOneRoll, ulong possibilitesToHere, WinTracker winTracker)
{
    if (playerOneRoll && playerTwoScore >= 21)
    {
        winTracker.PlyaerTwoWins += possibilitesToHere;
        return;
    }
    if (!playerOneRoll && playerOneScore >= 21)
    {
        winTracker.PlyaerOneWins += possibilitesToHere;
        return;
    }

    foreach (var (roll, times) in possibilityDict)
    {
        if (playerOneRoll)
        {
            var newPosition = ((playerOnePosition + roll - 1) % 10) + 1;
            RollForPlayer(playerOneScore + newPosition, playerTwoScore, newPosition, playerTwoPosition, !playerOneRoll, possibilitesToHere * times, winTracker);
        }
        else
        {
            var newPosition = ((playerTwoPosition + roll - 1) % 10) + 1;
            RollForPlayer(playerOneScore, playerTwoScore + newPosition, playerOnePosition, newPosition, !playerOneRoll, possibilitesToHere * times, winTracker);
        }
    }
}

class WinTracker
{
    public ulong PlyaerOneWins { get; set; } = 0;
    public ulong PlyaerTwoWins { get; set; } = 0;
}

class Die
{
    private int Position = 0;
    public int Rolls = 0;

    public int Roll()
    {
        if (Position == 100) Position = 1;
        else Position++;
        Rolls++;
        return Position;
    }
}

class Player
{
    public int Score { get; set; } = 0;
    public int Position { get; set; }

    public void Move(int spaces)
    {
        Position = ((Position + spaces - 1) % 10) + 1;
        Score += Position;
    }
}
