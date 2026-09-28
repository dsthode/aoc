package info.dsthode.aoc

import info.dsthode.aoc.util.WordTree
import java.nio.file.Files
import java.util.stream.Stream
import kotlin.io.path.Path
import kotlin.streams.toList

val numbers = listOf("one", "two", "three", "four", "five", "six", "seven", "eight", "nine")
val mapping = mapOf(
  "one" to "1",
  "two" to "2",
  "three" to "3",
  "four" to "4",
  "five" to "5",
  "six" to "6",
  "seven" to "7",
  "eight" to "8",
  "nine" to "9"
)
fun getLines(file: String): Stream<String> {
  val bufReader = Files.newBufferedReader(Path(file))
  return bufReader.lines()
}

fun part1(file: String) {
  val calibrations =
    getLines(file).map { line -> line.replace(Regex("[^0-9]"), "") }
      .map { value -> "${value.toList().first()}${value.toList().last()}" }
      .mapToInt { value -> value.toInt() }.toList()
  println("Part 1 results: ${calibrations.sum()}")
}

fun numberAsStringToNumber(value: String, tree: WordTree): String {
  var windowStart = 0
  var windowEnd = 1
  var newLine = StringBuffer()
  val chars = value.toCharArray()
  while (windowEnd < value.length) {
    if (tree.hasWord(value.substring(windowStart, windowEnd))) {
      windowEnd += 1
      val map = mapping[value.substring(windowStart, windowEnd)]
      if (map != null) {
        newLine.append(map)
      }
    } else {
      windowStart += 1
      windowEnd = windowStart + 1
    }
  }
  return value
}

fun part2(file: String) {
  val wordTree = WordTree(numbers)
  val calibrations = getLines(file).map { line -> numberAsStringToNumber(line, wordTree) }
    .map { line -> line.replace(Regex("[^0-9]"), "") }
    //.map { line -> println(line); line }
    .map { value -> "${value.toList().first()}${value.toList().last()}" }
    .mapToInt { value -> value.toInt() }.toList()
  println("Part 2 results: ${calibrations}")
}

fun main() {
  //part1()
  part2("input2.txt")
}
