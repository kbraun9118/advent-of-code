fun main() {
    val regex = """Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((\w+,?\s?)+)""".toRegex()

    val tunnels = readFile()
        .mapNotNull { regex.find(it) }
        .map { it.groupValues }
        .associate { it[1] to Tunnel(it[2].toInt(), it[3].split(", ")) }
    tunnels.forEach { (key, value) -> value.populateTunnels(key, tunnels) }

    printOutput(part1(tunnels), part2(tunnels))
}

fun part1(
    tunnels: Tunnels,
    start: String = "AA",
    remainingTime: Int = 30,
    remaining: Tunnels = tunnels.filterValues { it.flowRate > 0 },
): Int {
    if (remainingTime <= 0 || remaining.isEmpty()) {
        return 0
    }

    return remaining.maxOf { (to, _) ->
        val timeLeft = remainingTime - (tunnels[start]!!.pathsTo[to]!!.count() + 1)

        timeLeft * tunnels[to]!!.flowRate + part1(tunnels, to, timeLeft, remaining.filterKeys { it != to })
    }
}

fun part2(tunnels: Tunnels): Int {
    val lookup = mutableMapOf<Tunnels, Int>()
    val positiveFlow = tunnels.filterValues { it.flowRate > 0 }.toList()
    return positiveFlow.subsets()
        .map { it.toMap() to positiveFlow.filter { inner -> it.contains(inner).not() }.toMap() }
        .maxOf { (me, elephant) ->
            lookup.computeIfAbsent(me) {
                part1(tunnels, remainingTime = 26, remaining = me)
            } + lookup.computeIfAbsent(elephant) {
                part1(tunnels, remainingTime = 26, remaining = elephant)
            }
        }
}

typealias Tunnels = Map<String, Tunnel>


data class Tunnel(
    val flowRate: Int,
    val paths: List<String>,
) {
    lateinit var pathsTo: Map<String, List<String>>

    fun populateTunnels(current: String, tunnels: Tunnels) {
        val prev = tunnels.keys.associateWith { null }.toMutableMap<String, String?>()
        val dist = tunnels.keys.associateWith { Int.MAX_VALUE }.toMutableMap()
        val rest = tunnels.keys.toMutableSet()
        dist[current] = 0

        while (rest.isNotEmpty()) {
            val u = rest.minBy { dist[it]!! }
            rest -= u

            for (neighbor in tunnels[u]!!.paths.filter { rest.contains(it) }) {
                val alt = dist[u]!! + 1
                if (alt < dist[neighbor]!!) {
                    dist[neighbor] = alt
                    prev[neighbor] = u
                }
            }
        }

        val pathsTo = mutableMapOf<String, List<String>>()

        for (end in tunnels.filterValues { it.flowRate > 0 }.keys) {
            val path = mutableListOf<String>()
            var next = end
            while (next != current) {
                path += next
                next = prev[next]!!
            }
            pathsTo[end] = path.reversed()
        }
        this.pathsTo = pathsTo
    }
}
