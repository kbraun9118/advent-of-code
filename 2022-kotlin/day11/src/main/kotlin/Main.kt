import java.math.BigInteger

fun main() {
    val monkeys1 = readFile().chunked(7).map(Monkey::parse)
    val monkeys2 = readFile().chunked(7).map(Monkey::parse)
    val k = readFile()
        .chunked(7)
        .map { it[3].drop(21).toBigInteger() }
        .fold(BigInteger.ONE) { acc, i -> acc * i }

    printOutput(part1(monkeys1, k), part2(monkeys2, k))
}

fun part1(monkeys: List<Monkey>, k: BigInteger): Long {
    for (i in 0 until 20) {
        monkeys.forEach { inspectItems(it, monkeys, k) }
    }

    return monkeys
        .map { it.inspections }
        .sortedDescending()
        .take(2)
        .fold(1) { acc, i -> acc * i }
}

fun part2(monkeys: List<Monkey>, k: BigInteger): Long {
    for (i in 0 until 10_000) {
        monkeys.forEach { inspectItems(it, monkeys, k, false) }
    }

    return monkeys
        .map { it.inspections }
        .sortedDescending()
        .take(2)
        .fold(1) { acc, i -> acc * i }
}

fun inspectItems(
    monkey: Monkey,
    monkeys: List<Monkey>,
    k: BigInteger,
    worryDivided: Boolean = true,
) {
    monkey.inspections += monkey.items.size
    val items = monkey.items.take(monkey.items.size)
    monkey.items.clear()
    items
        .forEach { item ->
            val worry = monkey.operation(item) % k / if (worryDivided) {
                3.toBigInteger()
            } else {
                1.toBigInteger()
            }
            if (worry % monkey.test == BigInteger.ZERO) {
                monkeys[monkey.ifTrue].items.add(worry)
            } else {
                monkeys[monkey.ifFalse].items.add(worry)
            }
        }
}
//1981206786

data class Monkey(
    val id: Int,
    val items: MutableList<BigInteger>,
    val operation: (BigInteger) -> BigInteger,
    val test: BigInteger,
    val ifTrue: Int,
    val ifFalse: Int,
    var inspections: Long = 0
) {
    companion object {

        fun parse(input: List<String>): Monkey {
            val id = input[0].split(" ")[1].replace(":", "").toInt()
            val items = input[1].drop(18).split(", ").map(String::toBigInteger).toMutableList()
            val operation = if (input[2].contains("+")) {
                val split = input[2].drop(19).split(" + ")
                if (split[0] == "old" && split[1] == "old") {
                    { old: BigInteger -> old + old }
                } else if (split[0] == "old") {
                    { old: BigInteger -> old + split[1].toBigInteger() }
                } else if (split[1] == "old") {
                    { old: BigInteger -> old + split[0].toBigInteger() }
                } else {
                    { split[0].toBigInteger() + split[1].toBigInteger() }
                }
            } else {
                val split = input[2].drop(19).split(" * ")
                if (split[0] == "old" && split[1] == "old") {
                    { old: BigInteger -> old * old }
                } else if (split[0] == "old") {
                    { old: BigInteger -> old * split[1].toBigInteger() }
                } else if (split[1] == "old") {
                    { old: BigInteger -> old * split[0].toBigInteger() }
                } else {
                    { split[0].toBigInteger() * split[1].toBigInteger() }
                }
            }
            val test = input[3].drop(21).toBigInteger()
            val ifTrue = input[4].drop(29).toInt()
            val ifFalse = input[5].drop(30).toInt()
            return Monkey(id, items, operation, test, ifTrue, ifFalse)
        }
    }
}
