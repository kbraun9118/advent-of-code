import java.io.File

fun readFile(day: String): List<String> {
    val url = "../input/2022/${day}/input.txt"
    return File(url).readLines()
}

fun printOutput(part1: Any, part2: Any? = null) {
    println("Part 1: $part1")
    if (part2 != null) {
        println("Part 2: $part2")
    }
}

fun <T> Iterable<T>.chunkBy(func: (T) -> Boolean): List<List<T>> {
    val outer = mutableListOf<List<T>>()

    var inner = mutableListOf<T>()

    for (next in this) {
        if (func(next)) {
            outer += inner
            inner = mutableListOf()
        } else {
            inner += next
        }
    }

    outer += inner

    return outer
}

fun <T> Collection<T>.subsets(): List<List<T>> {
    val collection = this.toList()
    return sequence {
        for (i in 0 until (1 shl collection.count())) {
            val set = mutableListOf<T>()
            for (j in 0 until collection.count()) {
                if (i and (1 shl j) > 0) {
                    set.add(collection[j])
                }
            }
            yield(set)
        }
    }.toList()
}

class Graph<T>(
    xRange: IntRange,
    yRange: IntRange,
    defaultGen: () -> T? = { null },
) : Collection<T> {
    val minX = xRange.first
    val maxX = xRange.last

    val minY = yRange.first
    val maxY = yRange.last

    private val inner = yRange.map { xRange.map { defaultGen() }.toMutableList() }.toMutableList()

    operator fun get(x: Int, y: Int): T {
        return inner[y - minY][x - minX]!!
    }

    operator fun set(x: Int, y: Int, value: T?) {
        inner[y - minY][x - minX] = value
    }

    override val size: Int
        get() = inner.size * inner[0].size

    override fun isEmpty() = inner.isEmpty()

    override fun iterator() = inner.flatten().filterNotNull().iterator()

    override fun containsAll(elements: Collection<T>) = inner.flatten().containsAll(elements)

    override fun contains(element: T) = inner.flatten().contains(element)
}
