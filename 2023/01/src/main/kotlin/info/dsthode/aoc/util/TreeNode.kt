package info.dsthode.aoc.util

class TreeNode(val value: Char) {
  private val children: MutableList<TreeNode> = mutableListOf()

  fun addWord(word: String) {
    var child = children.find { child -> child.value == word[0] }
    if (child == null) {
      child = TreeNode(word[0])
      children.add(child)
    }
    if (word.length > 1) {
      child.addWord(word.substring(1))
    }
  }

  fun hasWord(word: String): Boolean {
    val child = children.find { node -> node.value == word[0] }
    if (child == null) {
      return false
    }
    if (word.length > 1) {
      return child.hasWord(word.substring(1))
    }
    return true
  }
}
