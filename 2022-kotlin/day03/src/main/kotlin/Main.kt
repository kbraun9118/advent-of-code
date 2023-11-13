fun main() {
    val lines = readFile()

    val part1 = lines
        .map { it.slice(0 until it.length / 2) to it.slice(it.length / 2 until it.length) }
        .map { (l, r) -> l.filter { r.contains(it) } }
        .sumOf { it.toSet().sumOf(::itemPriority) }

    val part2 = lines.chunked(3)
        .map { (f, s, t) -> f.filter { s.contains(it) }.filter { t.contains(it) }}
        .sumOf { it.toSet().sumOf(::itemPriority) }


    printOutput(part1, part2)
}

fun itemPriority(char: Char): Int {
    return if (char.code > 96) char.code - 96 else char.code - 38
}
