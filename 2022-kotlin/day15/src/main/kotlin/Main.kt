import java.util.LinkedList
import kotlin.math.absoluteValue

fun main() {
    val regex = """Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)""".toRegex()
    val areas = readFile()
        .map { regex.find(it)!!.groups }
        .map {
            Point(it[1]!!.value.toLong(), it[2]!!.value.toLong()) to
                    Point(it[3]!!.value.toLong(), it[4]!!.value.toLong())
        }.map { Area(it.first, it.second) }

    printOutput(part1(areas), part2(areas))
}

fun part1(areas: List<Area>): Int {
    val minX = areas.minOf { it.minX }
    val maxX = areas.maxOf { it.maxX }

    return (minX..maxX)
        .map { Point(it, 2000000) }
        .filter { areas.none { area -> area.beacon == it || area.sensor == it } }
        .count { areas.any { area -> area.contains(it) } }
}

fun part2(areas: List<Area>, searchArea: Long = 4000000L): Long {
    val (i, ranges) = (0..searchArea).map {
        areas.mergeAt(it)
    }.withIndex()
        .first { (_, ranges) ->
            ranges.none { inner ->
                (0..searchArea)
                    .fullyContainedWithin(inner)
            }
        }
    return (0..4000000L).first { ranges.none { inner -> inner.contains(it) } } * 4000000L + i
}

fun List<Area>.mergeAt(y: Long): List<LongRange> {
    val ranges = LinkedList(mapNotNull { it.rangeAt(y) })
    var first = ranges.pop()
    while (ranges.isNotEmpty() && ranges.any { first.canMerge(it) }) {
        val next = ranges.pop()
        val newRange = first.merge(next)
        if (newRange != null) {
            first = newRange
        } else {
            ranges.add(next)
        }
    }
    return listOf(first) + ranges
}

fun LongRange.merge(other: LongRange): LongRange? {
    return if (this.canMerge(other)) {
        minOf(this.first, other.first)..maxOf(this.last, other.last)
    } else {
        null
    }
}

fun LongRange.canMerge(other: LongRange): Boolean {
    return (this.first <= other.first && this.last >= other.first)
            || (other.first <= this.first && other.last >= this.first)
            || this.fullyContainedWithin(other)
            || other.fullyContainedWithin(this)
}

fun LongRange.fullyContainedWithin(other: LongRange): Boolean {
    return this.first >= other.first && this.last <= other.last
}

data class Point(
    val x: Long,
    val y: Long,
)

data class Area(
    val sensor: Point,
    val beacon: Point,
) {
    private val distance = (sensor.x - beacon.x).absoluteValue + (sensor.y - beacon.y).absoluteValue
    val maxX = sensor.x + distance
    val minX = sensor.x - distance

    fun contains(point: Point) = (sensor.x - point.x).absoluteValue + (sensor.y - point.y).absoluteValue <= distance

    fun rangeAt(y: Long): LongRange? {
        val distanceToY = (sensor.y - y).absoluteValue
        return if (distanceToY > distance) {
            null
        } else {
            val diff = distance - distanceToY
            (sensor.x - diff)..sensor.x + diff
        }
    }
}

//Sensor at x=20, y=1: closest beacon is at x=15, y=3
