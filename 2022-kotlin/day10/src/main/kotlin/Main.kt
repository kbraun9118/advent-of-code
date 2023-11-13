fun main() {
    val instructions = readFile().map(Instruction::parse)

    val cycles = mutableListOf(Cycle(1, 1))

    instructions.forEach { it.addToCycles(cycles) }

    val part1 = cycles.asSequence()
        .withIndex()
        .drop(20)
        .chunked(40)
        .sumOf { it[0].value.start * it[0].index }

    printOutput(part1, "\n" + part2(cycles))
}

fun part2(cycles: List<Cycle>): String {
    val sb = StringBuilder()
    for (i in 0..5) {
        for (j in 1..40) {
            val pixel = (cycles[j + (i * 40)].start - 1..cycles[j + (i * 40)].start + 1)
            if (pixel.contains(j - 1)) {
                sb.append('#')
            } else {
                sb.append('.')
            }
        }
        sb.append('\n')
    }
    return sb.toString()
}

sealed class Instruction {
    companion object {
        fun parse(string: String): Instruction {
            return if (string.startsWith("noop")) {
                Noop
            } else {
                AddX(string.split(" ")[1].toInt())
            }
        }
    }

    abstract fun addToCycles(cycles: MutableList<Cycle>)
}

object Noop : Instruction() {
    override fun addToCycles(cycles: MutableList<Cycle>) {
        cycles += Cycle(cycles.last().end, cycles.last().end)
    }
}

class AddX(private val xReg: Int) : Instruction() {
    override fun addToCycles(cycles: MutableList<Cycle>) {
        cycles += Cycle(cycles.last().end, cycles.last().end)
        cycles += Cycle(cycles.last().end, cycles.last().end + xReg)
    }

}

data class Cycle(val start: Int, val end: Int)
