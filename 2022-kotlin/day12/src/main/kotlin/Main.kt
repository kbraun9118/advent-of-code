fun main() {
    val lines = readFile("12")
    val graph = Graph<Step>(0 until lines[0].length, lines.indices)
    lines.flatMapIndexed { y, row ->
        row.mapIndexed { x, step ->
            val height = when (step) {
                'S' -> 0
                'E' -> 27
                else -> step.code - 96
            }
            Step(x, y, height)
        }
    }.forEach {
        graph[it.x, it.y] = it
    }
    graph.forEach { it.populateNeighbors(graph) }
    val start = graph.find { it.height == 0 }!!
    val end = graph.find { it.height == 27 }!!

    val part1 = pathTo(start, end, graph)

    val part2 = graph.filter { it.height == 1 }
        .minOfOrNull { pathTo(it, end, graph) }

    printOutput(part1, part2)
}

fun pathTo(start: Step, end: Step, graph: Graph<Step>): Int {
    val dist = graph.associateWith { Int.MAX_VALUE }.toMutableMap()
    val prev = graph.associateWith { null }.toMutableMap<_, Step?>()
    val queue = graph.toMutableList()
    dist[start] = 0

    while (queue.isNotEmpty()) {
        val u = queue.minBy { dist[it] ?: Int.MAX_VALUE }
        if (u == end || dist[u] == Int.MAX_VALUE) {
            break
        }
        queue.remove(u)

        for (neighbor in u.neighbors) {
            val alt = (dist[u] ?: (Int.MAX_VALUE - 1)) + 1
            if (alt < (dist[neighbor] ?: Int.MAX_VALUE)) {
                dist[neighbor] = alt
                prev[neighbor] = u
            }
        }
    }

    return dist[end] ?: -1
}

class Step(
    val x: Int,
    val y: Int,
    val height: Int,
    private val _neighbors: MutableSet<Step> = mutableSetOf()
) {
    val neighbors
        get() = _neighbors.filter {
            1 + if (height == 0) {
                1
            } else {
                height
            } >= if (it.height == 27) {
                26
            } else {
                it.height
            }
        }.toSet()

    fun populateNeighbors(graph: Graph<Step>) {
        var xRange = -1..1
        var yRange = -1..1
        if (x == graph.minX) {
            xRange = 0..xRange.max()
        }
        if (x == graph.maxX) {
            xRange = xRange.min()..0
        }
        if (y == graph.minY) {
            yRange = 0..yRange.max()
        }
        if (y == graph.maxY) {
            yRange = yRange.min()..0
        }

        for (xDiff in xRange) {
            if (xDiff != 0) {
                _neighbors.add(graph[x + xDiff, y])
            }
        }
        for (yDiff in yRange) {
            if (yDiff != 0) {
                _neighbors.add(graph[x, y + yDiff])
            }
        }
    }
}
