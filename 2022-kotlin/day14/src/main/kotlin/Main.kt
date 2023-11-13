fun main() {
    val ranges = readFile().map { line ->
        line.split(" -> ")
            .map {
                it
                    .split(',')
                    .map(String::toInt)
                    .zipWithNext()
                    .first()
            }
            .map { Point(it.first, it.second) }
            .zipWithNext()
    }

    val rocks1 = createRocks(ranges)
    val rocks2 = createRocks(ranges)

    placeSand(rocks1)
    placeSand(rocks2, true)

    printOutput(
        rocks1.filter { it.value == Placment.SAND }.count(),
        rocks2.filter { it.value == Placment.SAND }.count(),
    )
}

private fun createRocks(ranges: List<List<Pair<Point, Point>>>) = ranges
    .flatten()
    .flatMap { (l, r) -> l..r }
    .associateWith { Placment.ROCK }
    .toMutableMap()

fun placeSand(rocks: MutableMap<Point, Placment>, part2: Boolean = false) {
    val maxY = rocks.keys.maxBy { it.y }.y + 2
    while (true) {
        var current = Point(500, 0)
        inner@ while (true) {
            if (current.y + 1 == maxY) {
                if (part2) {
                    rocks[current] = Placment.SAND
                    break@inner
                } else {
                    return
                }
            } else if (!rocks.containsKey(Point(current.x, current.y + 1))) {
                current = Point(current.x, current.y + 1)
            } else if (!rocks.containsKey(Point(current.x - 1, current.y + 1))) {
                current = Point(current.x - 1, current.y + 1)
            } else if (!rocks.containsKey(Point(current.x + 1, current.y + 1))) {
                current = Point(current.x + 1, current.y + 1)
            } else {
                rocks[current] = Placment.SAND
                if (current == Point(500, 0)) {
                    return
                }
                break@inner
            }
        }
    }
}

data class Point(
    val x: Int,
    val y: Int
) {
    operator fun rangeTo(other: Point): Sequence<Point> {
        return sequence {
            var current = this@Point
            while (current != other) {
                yield(current)
                current = if (current.y == other.y) {
                    if (current.x > other.x) {
                        Point(current.x - 1, current.y)
                    } else {
                        Point(current.x + 1, current.y)
                    }
                } else {
                    if (current.y > other.y) {
                        Point(current.x, current.y - 1)
                    } else {
                        Point(current.x, current.y + 1)
                    }
                }
            }
            yield(current)
        }
    }
}

enum class Placment {
    ROCK,
    SAND
}
