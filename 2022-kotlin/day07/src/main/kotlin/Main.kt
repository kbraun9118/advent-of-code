fun main() {
    val lines = readFile("07")

    val fs = parseFS(lines)

    printOutput(part1(fs), part2(fs))
}

fun part1(root: Directory): Long {
    return root.files
        .filter { it.size <= 100_000 }
        .filterIsInstance<Directory>()
        .sumOf { it.size } +
            root.files
                .filterIsInstance<Directory>()
                .sumOf { part1(it) }
}

fun part2(root: Directory): Long {
    return allDirs(root)
        .map { it.size }
        .filter { 70_000_000 - root.size + it >= 30_000_000 }
        .min()
}

fun allDirs(root: Directory): List<Directory> {
    return root.files
        .filterIsInstance<Directory>()
        .flatMap { allDirs(it) }
        .toMutableList() +
            root.files.filterIsInstance<Directory>()
}

fun parseFS(lines: List<String>): Directory {
    val root = Directory("/")
    var current = root

    for (line in lines.slice(1 until lines.size)) {
        if (line.startsWith("$ ls")) {
            continue
        }
        if (line.startsWith("$ cd")) {
            current = if (line.contains("..")) {
                current.parent!!
            } else {
                current.files.first { it.name == line.split(' ')[2] } as Directory
            }
        } else {
            current.files += FS.parse(line, current)
        }
    }

    return root
}

sealed class FS(val name: String) {

    abstract val size: Long

    companion object {
        fun parse(input: String, parent: Directory): FS {
            return if (input.startsWith("dir ")) {
                Directory(input.split(' ')[1], parent)
            } else {
                val split = input.split(' ')
                File(split[1], split[0].toLong())
            }
        }
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as FS

        if (name != other.name) return false

        return true
    }

    override fun hashCode(): Int {
        return name.hashCode()
    }


}

class File(name: String, override val size: Long) : FS(name)

class Directory(name: String, val parent: Directory? = null, val files: MutableSet<FS> = mutableSetOf()) : FS(name) {
    override val size: Long get() = files.sumOf { it.size }
}
