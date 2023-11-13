fun main() {
    val elves = readFile()
        .chunkBy { it == "" }
        .map { it.sumOf { inner -> inner.toInt() } }
        .sortedDescending()

    printOutput(elves[0], elves.take(3).sum())
}
