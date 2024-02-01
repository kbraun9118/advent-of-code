import java.util.Stack

fun main() {
    val lines = readFile("05")

    val (config, instructionsInput) = lines.chunkBy { it == "" }

    var stacks = parseStacks(config)
    val instructions = instructionsInput.map(::parseInstruction)

    part1(stacks, instructions)

    val part1 = stacks.map { it.peek() }.joinToString(separator = "")

    stacks = parseStacks(config)

    part2(stacks, instructions);

    printOutput(
        part1,
        stacks.map { it.peek() }.joinToString(separator = ""),
    )
}

fun part1(stacks: List<Stack<Char>>, instructions: List<Instruction>) {
    for (instruction in instructions) {
        for (i in 0 until instruction.amount) {
            stacks[instruction.to - 1].push(stacks[instruction.from - 1].pop())
        }
    }
}

fun part2(stacks: List<Stack<Char>>, instructions: List<Instruction>) {
    for (instruction in instructions) {
        val buffer = mutableListOf<Char>()
        for (i in 0 until instruction.amount) {
            buffer += stacks[instruction.from - 1].pop()
        }
        buffer.reversed().forEach { stacks[instruction.to - 1].push(it) }
    }
}

fun parseStacks(config: List<String>): List<Stack<Char>> {
    val stacks = mutableListOf<Stack<Char>>()

    for (i in 0 until config.last().last().digitToInt()) {
        stacks += Stack<Char>()
    }

    for (i in config.size - 2 downTo 0) {
        for (j in 1 until config[i].length step 4) {
            if (config[i][j] != ' ') {
                stacks[(j - 1) / 4] += config[i][j]
            }
        }
    }

    return stacks
}

fun parseInstruction(instruction: String): Instruction {
    val regex = "move (\\d+) from (\\d+) to (\\d+)".toRegex()

    val matches = regex.find(instruction)

    return Instruction(
        matches!!.groups[1]!!.value.toInt(),
        matches.groups[2]!!.value.toInt(),
        matches.groups[3]!!.value.toInt(),
    )
}

data class Instruction(
    val amount: Int,
    val from: Int,
    val to: Int,
)
