import scala.collection.mutable.HashMap
import scala.collection.mutable.HashSet
import scala.io.Source
import scala.collection.mutable.ArrayBuffer

def count_antinodes_no_resonance(
    antennas: HashMap[Int, ArrayBuffer[(Int, Int)]],
    rows: Int,
    cols: Int
): Int = {
  var antinode_positions = Set.empty[(Int, Int)]
  for (antennas <- antennas.values)
    for (((i1, j1), num) <- antennas.zipWithIndex)
      for ((i2, j2) <- antennas.drop(num + 1))
        val i_a1 = 2 * i1 - i2
        val j_a1 = 2 * j1 - j2
        val i_a2 = 2 * i2 - i1
        val j_a2 = 2 * j2 - j1
        if ((0 until rows).contains(i_a1) && (0 until cols).contains(j_a1))
          antinode_positions += (i_a1, j_a1)
        if ((0 until rows).contains(i_a2) && (0 until cols).contains(j_a2))
          antinode_positions += (i_a2, j_a2)
  antinode_positions.size
}

def count_antinodes_with_resonance(
    antennas: HashMap[Int, ArrayBuffer[(Int, Int)]],
    rows: Int,
    cols: Int
): Int = {
  var antinode_positions = Set.empty[(Int, Int)]
  for (antennas <- antennas.values)
    for (((i1, j1), num) <- antennas.zipWithIndex)
      if (antennas.size > 1)
        antinode_positions += (i1, j1)
      for ((i2, j2) <- antennas.drop(num + 1))
        var vi = i2 - i1
        var vj = j2 - j1
        var new_i = i2 + vi
        var new_j = j2 + vj
        while ((0 until rows).contains(new_i) && (0 until cols).contains(new_j))
          antinode_positions += (new_i, new_j)
          new_i += vi
          new_j += vj
        vi *= -1
        vj *= -1
        new_i = i1 + vi
        new_j = j1 + vj
        while ((0 until rows).contains(new_i) && (0 until cols).contains(new_j))
          antinode_positions += (new_i, new_j)
          new_i += vi
          new_j += vj
  antinode_positions.size
}

@main def run(): Unit = {
  var antennas = HashMap.empty[Int, ArrayBuffer[(Int, Int)]]
  var rows = 0
  var cols = 0

  for (
    (line, i) <- Source
      .fromFile("day8.txt")
      .getLines()
      .map(_.getBytes)
      .zipWithIndex
  )
    rows = i + 1
    cols = line.size
    for ((frequency, j) <- line.zipWithIndex)
      if (frequency != '.')
        antennas
          .getOrElseUpdate(frequency, ArrayBuffer.empty[(Int, Int)]) += ((i, j))

  val part1 = count_antinodes_no_resonance(antennas, rows, cols)
  val part2 = count_antinodes_with_resonance(antennas, rows, cols)
  println(s"$part1, $part2")
}
