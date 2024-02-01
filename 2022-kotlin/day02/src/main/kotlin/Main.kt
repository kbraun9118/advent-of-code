fun main() {
    val lines = readFile("02")
        .map { it[0] to it[2] }
    val part1 = lines
        .map { (left, right) -> left.toRPS() to right.toRPS() }
        .sumOf { (opponent, self) -> (self plays opponent).score + self.score }

    val part2 = lines
        .map { (left, right) -> left.toRPS() to right.toOutcome() }
        .sumOf { (opponent, outcome) -> (outcome shouldPlay opponent).score + outcome.score }

    printOutput(part1, part2)
}

fun Char.toRPS() = when (this) {
    'A', 'X' -> RPS.ROCK
    'B', 'Y' -> RPS.PAPER
    else -> RPS.SCISSORS
}

enum class RPS(val score: Int) {
    ROCK(1),
    PAPER(2),
    SCISSORS(3);

    infix fun plays(other: RPS) = when (this) {
        ROCK -> when (other) {
            ROCK -> Outcome.DRAW
            PAPER -> Outcome.LOSE
            SCISSORS -> Outcome.WIN
        }

        PAPER -> when (other) {
            ROCK -> Outcome.WIN
            PAPER -> Outcome.DRAW
            SCISSORS -> Outcome.LOSE
        }

        SCISSORS -> when (other) {
            ROCK -> Outcome.LOSE
            PAPER -> Outcome.WIN
            SCISSORS -> Outcome.DRAW
        }

    }
}

fun Char.toOutcome() = when (this) {
    'X' -> Outcome.LOSE
    'Y' -> Outcome.DRAW
    else -> Outcome.WIN
}

enum class Outcome(val score: Int) {
    WIN(6),
    DRAW(3),
    LOSE(0);

    infix fun shouldPlay(rps: RPS) = when (this) {
        WIN -> when (rps) {
            RPS.ROCK -> RPS.PAPER
            RPS.PAPER -> RPS.SCISSORS
            RPS.SCISSORS -> RPS.ROCK
        }

        LOSE -> when (rps) {
            RPS.ROCK -> RPS.SCISSORS
            RPS.PAPER -> RPS.ROCK
            RPS.SCISSORS -> RPS.PAPER
        }

        else -> rps
    }
}
