import scala.annotation.tailrec
import scala.io.Source

enum Direction {
  case North
  case East
  case South
  case West

  def velocity: (Int, Int) = this match {
    case North => (-1, 0)
    case East  => (0, 1)
    case South => (1, 0)
    case West  => (0, -1)
  }

  def turnClockwise: Direction = this match {
    case North => East
    case East  => South
    case South => West
    case West  => North
  }
}

class Position(var i: Int, var j: Int):
  def getAdjacent(direction: Direction) = {
    val (vi, vj) = direction.velocity
    Position(i + vi, j + vj)
  }

  def move(direction: Direction) = {
    val (vi, vj) = direction.velocity
    i += vi;
    j += vj;
  }

class Guard(var position: Position, var direction: Direction):
  def lookAhead = position.getAdjacent(direction)
  def move() = position.move(direction)
  def turnClockwise() = direction = direction.turnClockwise
  def copy: Guard = Guard(Position(position.i, position.j), direction)

enum LabEntry {
  case Unenterable
  case Vacant
  case Starting
  case Visited
  case Obstacle(var hitInDirection: Set[Direction], var roundUpdated: Int)
}

object LabEntry {
  def fromChar(c: Byte): LabEntry = c match {
    case '.' => Vacant
    case '^' => Starting
    case '#' => Obstacle(Set(), 0)
    case _   => throw new IllegalArgumentException(s"Invalid character: $c")
  }
}

class LabGrid(layoutSpec: Array[Array[Byte]]):
  // Pad the layout with Unenterable
  var layout =
    val rows = layoutSpec.map(row => {
      val mapped_row = row.map(x => LabEntry.fromChar(x))
      LabEntry.Unenterable +: mapped_row :+ LabEntry.Unenterable
    })
    val filler = Array.fill(rows(0).length)(LabEntry.Unenterable)
    filler +: rows :+ filler

    // Identify the starting index of the guard
  var start = layout.zipWithIndex
    .map((row, i) => (row.zipWithIndex.map((entry, j) => (entry, i, j))))
    .flatten
    .find((entry, i, j) => entry == LabEntry.Starting)
    .map((_, start_i, start_j) => Position(start_i, start_j))
    .get

  def apply(position: Position): LabEntry =
    layout(position.i)(position.j)

  def update(position: Position, entry: LabEntry) =
    layout(position.i)(position.j) = entry

class Simulation(layoutSpec: Array[Array[Byte]]):
  var labGrid = LabGrid(layoutSpec)
  var guard = Guard(labGrid.start, Direction.North)
  var visitedCounter = 1;
  var cycleCounter = 0;
  var round = 0;

  def markGuardLocationVisited() =
    visitedCounter += 1
    labGrid(guard.position) = LabEntry.Visited

  def simulateObstacle() = {
    var newGuard = guard.copy
    var obstaclePosition = newGuard.lookAhead
    var original = labGrid(obstaclePosition);
    labGrid(obstaclePosition) = LabEntry.Obstacle(Set(), round)

    @tailrec
    def run(): Boolean =
      labGrid(newGuard.lookAhead) match
        case LabEntry.Unenterable => false
        case LabEntry.Vacant | LabEntry.Visited | LabEntry.Starting =>
          newGuard.move()
          run()
        case LabEntry.Obstacle(hit_in_direction, round_updated) =>
          if (round_updated == round)
            if (hit_in_direction.contains(newGuard.direction)) true
            else
              labGrid(newGuard.lookAhead) = LabEntry.Obstacle(
                hit_in_direction + newGuard.direction,
                round_updated
              )
              newGuard.turnClockwise()
              run()
          else
            labGrid(newGuard.lookAhead) =
              LabEntry.Obstacle(Set(newGuard.direction), round)
            newGuard.turnClockwise()
            run()

    val out = run()
    labGrid(obstaclePosition) = original
    out
  }

  def run(): (Int, Int) =
    @tailrec
    def runInner(): Unit =
      labGrid(guard.lookAhead) match
        case LabEntry.Starting | LabEntry.Visited =>
          guard.move()
          runInner()
        case LabEntry.Unenterable => ()
        case LabEntry.Vacant =>
          if (simulateObstacle())
            cycleCounter += 1
          round += 1
          guard.move()
          markGuardLocationVisited()
          runInner()
        case LabEntry.Obstacle(_, _) =>
          guard.turnClockwise()
          runInner()
    runInner()
    (visitedCounter, cycleCounter)

@main def run(): Unit = {
  val layoutSpec: Array[Array[Byte]] = Source
    .fromFile("day6.txt")
    .getLines()
    .map(_.getBytes)
    .toArray

  val (visitedCounter, cycleCounter) = Simulation(layoutSpec).run()
  println(s"$visitedCounter, $cycleCounter")
}
