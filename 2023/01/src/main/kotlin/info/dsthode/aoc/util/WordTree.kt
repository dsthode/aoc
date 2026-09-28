package info.dsthode.aoc.util

class WordTree(words: List<String>) {
  private val root = TreeRoot()

  init {
    words.forEach { word -> root.addWord(word) }
  }

  fun hasWord(word: String): Boolean {
    return root.hasWord(word)
  }
}
