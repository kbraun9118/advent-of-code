fun main() {
    val trees = readFile().map { it.map { inner -> inner.digitToInt() } }

    printOutput(part1(trees), part2(trees))
}

fun part1(trees: List<List<Int>>): Int {

    return trees.flatMapIndexed { i, row ->
        row.mapIndexed { j, tree ->
            if (
                trees[i].slice(0 until j).all { it < tree }
                || trees[i].slice(j + 1 until trees.size).all { it < tree }
                || trees.slice(0 until i).all { it[j] < tree }
                || trees.slice(i + 1 until trees.size).all { it[j] < tree }
            ) {
                1
            } else {
                0
            }
        }
    }.sum()
}

fun part2(trees: List<List<Int>>): Int {
    return trees.withIndex()
        .drop(1)
        .dropLast(1)
        .flatMap { (i, row) ->
            row.withIndex()
                .drop(1)
                .dropLast(1)
                .map { (j, tree) ->
                    val first = trees[i].slice(0 until j).reversed()
                    val second = trees[i].slice(j + 1 until trees.size)
                    val third = trees.slice(0 until i).reversed().map { it[j] }
                    val fourth = trees.slice(i + 1 until trees.size).map { it[j] }

                    val firstView = first.takeWhile { it < tree }
                    val secondView = second.takeWhile { it < tree }
                    val thirdView = third.takeWhile { it < tree }
                    val fourthView = fourth.takeWhile { it < tree }

                    if (firstView.count() == first.count()) {
                        firstView.count()
                    } else {
                        firstView.count() + 1
                    } * if (secondView.count() == second.count()) {
                        secondView.count()
                    } else {
                        secondView.count() + 1
                    } * if (thirdView.count() == third.count()) {
                        thirdView.count()
                    } else {
                        thirdView.count() + 1
                    } * if (fourthView.count() == fourth.count()) {
                        fourthView.count()
                    } else {
                        fourthView.count() + 1
                    }
                }
        }.max()
}
