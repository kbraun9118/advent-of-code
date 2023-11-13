fun main() {
    val pattern1 = generateSequence { readFile()[0].map { it } }.flatten()
    val pattern2 = generateSequence { readFile()[0].map { it } }.flatten()

    printOutput(simulation(pattern1, 2022), simulation(pattern2, 2022 * 2))
}

fun simulation(pattern: Sequence<Char>, amount: Long): Long {
    val restedRocks = mutableSetOf<Point>()
    val patternIterator = pattern.iterator()
    val shapeIterator = Shape.order().iterator()
    for (i in 0 until amount) {
        val shape = shapeIterator.next()
        val maxY = restedRocks.maxOfOrNull { it.y } ?: 0
        var currentShape = nextPoints(shape, maxY)
        while (true) {
            val pushedShape = if (patternIterator.next() == '>') {
                currentShape.map { Point(it.x + 1, it.y) }
            } else {
                currentShape.map { Point(it.x - 1, it.y) }
            }
            if (pushedShape.all { it.x in 0..6 && restedRocks.contains(it).not() }) {
                currentShape = pushedShape
            }
            val droppedShape = currentShape.map { Point(it.x, it.y - 1) }
            if (droppedShape.any { restedRocks.contains(it)}) {
                break
            }
            currentShape = droppedShape
            if (droppedShape.any { it.y == 0}) {
                break
            }
        }
        restedRocks.addAll(currentShape)
    }
        printRocks(restedRocks)
    return (restedRocks.maxOfOrNull { it.y.toLong() } ?: 0) + 1
}

fun printRocks(restedRocks: Set<Point>) {
    println()
    for (y in restedRocks.maxOf { it.y } downTo 0) {
        print("|")
        for (x in 0..6) {
            if (restedRocks.contains(Point(x, y))) {
                print('#')
            } else {
                print('.')
            }
        }
        println('|')
    }
    println("+-------+")
}

fun nextPoints(shape: Shape, maxY: Int) = when (shape) {
    Shape.Plus -> listOf(
        Point(3, maxY + 4),
        Point(3, maxY + 5),
        Point(2, maxY + 5),
        Point(4, maxY + 5),
        Point(3, maxY + 6),
    )

    Shape.Square -> listOf(
        Point(2, maxY + 4),
        Point(2, maxY + 5),
        Point(3, maxY + 4),
        Point(3, maxY + 5),
    )

    Shape.Tall -> listOf(
        Point(2, maxY + 4),
        Point(2, maxY + 5),
        Point(2, maxY + 6),
        Point(2, maxY + 7),
    )

    Shape.Horizontal -> listOf(
        Point(2, maxY + 4),
        Point(3, maxY + 4),
        Point(4, maxY + 4),
        Point(5, maxY + 4),
    )

    Shape.Corner -> listOf(
        Point(2, maxY + 4),
        Point(3, maxY + 4),
        Point(4, maxY + 4),
        Point(4, maxY + 5),
        Point(4, maxY + 6),
    )
}

enum class Shape {
    Horizontal,
    Tall,
    Corner,
    Square,
    Plus;

    companion object {
        fun order(): Sequence<Shape> {
            return generateSequence { listOf(Horizontal, Plus, Corner, Tall, Square) }.flatten()
        }
    }
}

data class Point(val x: Int, val y: Int)
