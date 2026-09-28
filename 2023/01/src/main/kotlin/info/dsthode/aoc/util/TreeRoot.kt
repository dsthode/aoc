package info.dsthode.aoc.util

class TreeRoot {
  private val children: MutableList<TreeNode> = mutableListOf()

  fun addWord(word: String) {
    var child = children.find { node -> node.value == word[0] }
    if (child == null) {
      child = TreeNode(word[0])
      children.add(child)
    }
    child.addWord(word.substring(1))
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
