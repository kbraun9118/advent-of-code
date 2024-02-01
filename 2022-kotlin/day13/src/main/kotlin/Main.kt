fun main() {
    val packets =
        readFile("13")
            .chunked(3)
            .map {
                it.dropLastWhile(String::isEmpty)
                    .map(Packet::parse)
                    .zipWithNext()
                    .first()
            }

    val part1 = packets.withIndex().filter { (_, packets) -> packets.first < packets.second }
        .sumOf { (i, _) -> i + 1 }

    val dividerPackets = listOf(Packet.parse("[[2]]"), Packet.parse("[[6]]"))
    val part2 = (readFile("13")
        .asSequence()
        .filter(String::isNotEmpty)
        .map(Packet::parse) + dividerPackets)
        .sorted()
        .withIndex()
        .filter { (_, p) -> dividerPackets.contains(p) }
        .map { (i) -> i }
        .fold(1) { acc, next -> acc * (next + 1) }

    printOutput(part1, part2)
}

sealed class Packet : Comparable<Packet> {
    companion object {
        fun parse(input: String): Packet {
            val (out) = parseIterative(input)
            return out
        }

        private fun parseIterative(input: String): Pair<Packet, String> {
            return if (input.startsWith("[")) {
                var rest = input.drop(1)
                val packets = mutableListOf<Packet>()
                while (rest.startsWith("]").not()) {
                    val parsed = parseIterative(rest)
                    rest = parsed.second
                    if (rest.startsWith(',')) {
                        rest = rest.drop(1)
                    }
                    packets += parsed.first
                }
                ListPacket(packets) to rest.drop(1)
            } else {
                val num = input.takeWhile { it.isDigit() }
                Integer(num.toInt()) to input.drop(num.length)
            }
        }
    }

    abstract override operator fun compareTo(other: Packet): Int
}

data class Integer(
    val value: Int,
) : Packet() {
    override fun compareTo(other: Packet): Int {
        return when (other) {
            is Integer -> this.value.compareTo(other.value)
            else -> ListPacket(listOf(this)).compareTo(other)
        }
    }
}

data class ListPacket(
    val value: List<Packet>
) : Packet() {
    override fun compareTo(other: Packet): Int {
        return when (other) {
            is Integer -> this.compareTo(ListPacket(listOf(other)))
            is ListPacket -> value.zip(other.value)
                .map { it.first.compareTo(it.second) }
                .firstOrNull { it != 0 } ?: value.size.compareTo(other.value.size)
        }
    }
}
