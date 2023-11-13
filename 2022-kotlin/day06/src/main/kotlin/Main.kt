fun main() {
    val input = readFile()[0]

    val part1 = input
        .withIndex()
        .asSequence()
        .windowed(4)
        .map { items -> items[0].index to items.map { it.value }.toSet() }
        .filter { (_, set) -> set.size == 4 }
        .map { (i, _) -> i }
        .first() + 4

    val part2 = input
        .withIndex()
        .asSequence()
        .windowed(14)
        .map { items -> items[0].index to items.map { it.value }.toSet() }
        .filter { (_, set) -> set.size == 14 }
        .map { (i, _) -> i }
        .first() + 14

    printOutput(part1, part2)
}
