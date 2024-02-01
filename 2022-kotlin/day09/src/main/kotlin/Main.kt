import kotlin.math.absoluteValue

fun main() {
    val instructions = readFile("09")
        .map(Instruction::fromString)
    printOutput(moveRope(instructions), moveRope(instructions, 9))
}

fun moveRope(instructions: List<Instruction>, segmentLength: Int = 1): Int {
    val hasVisited = mutableSetOf<Pair<Int, Int>>()
    val segments = (0..segmentLength).map { 0 to 0 }.toMutableList()
    for (instruction in instructions) {
        for (i in 0 until instruction.distance) {
            var (headX, headY) = segments[0]
            when (instruction.direction) {
                "U" -> headY++
                "D" -> headY--
                "R" -> headX++
                else -> headX--
            }
            segments[0] = headX to headY
            for (j in 1..segmentLength) {
                segments[j] = moveWith(segments[j - 1], segments[j])
            }
            hasVisited += segments[segmentLength]
        }
    }
    return hasVisited.size
}

fun moveWith(prev: Pair<Int, Int>, current: Pair<Int, Int>): Pair<Int, Int> {
    val (headX, headY) = prev
    var (tailX, tailY) = current
    if ((headY - tailY).absoluteValue == 2) {
        if ((headX - tailX).absoluteValue == 1) {
            tailX = headX
        } else if ((headX - tailX).absoluteValue == 2) {
            tailX = if (tailX > headX) {
                tailX - 1
            } else {
                tailX + 1
            }
        }
        tailY = if (tailY > headY) {
            tailY - 1
        } else {
            tailY + 1
        }
    } else if ((headX - tailX).absoluteValue == 2) {
        if ((headY - tailY).absoluteValue == 1) {
            tailY = headY
        }
        tailX = if (tailX > headX) {
            tailX - 1
        } else {
            tailX + 1
        }
    }
    return tailX to tailY
}

data class Instruction(
    val direction: String,
    val distance: Int
) {
    companion object {
        fun fromString(string: String): Instruction {
            val split = string.split(" ")
            return Instruction(split[0], split[1].toInt())
        }
    }
}


