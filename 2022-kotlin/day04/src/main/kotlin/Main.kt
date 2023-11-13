fun main() {

    val pairs = readFile()
        .map { it.split(',') }
        .map { it.map { inner -> inner.split('-').map(String::toInt) } }
        .map { it[0][0]..it[0][1] to it[1][0]..it[1][1] }

    val part1 = pairs.count { (l, r) -> l.all(r::contains) || r.all(l::contains) }
    val part2 = pairs.count { (l, r) -> l.intersect(r).isNotEmpty() }

    printOutput(part1, part2)
}
