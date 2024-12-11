import java.io.IOException;

public class Day10 extends Grid<Byte> {
    Day10(String filename) throws IOException {
        super(filename, (byte) 127);
    }

    public static void main(String[] args) {
        if (args.length > 0) {
            String filename = args[0];
            try {
                Day10 grid = new Day10(filename);
                int score = grid.getAllScore();
                int paths = grid.countAllPaths();
                System.out.println(score + ", " + paths);
            } catch (IOException e) {
                e.printStackTrace();
            }
        } else {
            System.out.println("No filename provided.");
        }
    }

    int getAllScore() {
        Integer round = 1;
        Grid<Integer> visited = new Grid<Integer>(0, this.rows, this.cols);
        Integer score = 0;
        for (int i = 1; i < this.rows - 1; i++) {
            for (int j = 1; j < this.cols - 1; j++) {
                Index2D position = new Index2D(i, j);
                if (get(position) == 0) {
                    score += getScore(position, visited, round);
                    round += 1;
                }
            }
        }

        return score;
    }

    int getScore(Index2D position, Grid<Integer> visited, int round) {
        if (visited.get(position) == round) {
            return 0;
        } else if (get(position) == 9) {
            visited.set(position, round);
            return 1;
        } else {
            int out = 0;
            for (Index2D offset : NEIGHBOR_OFFSETS) {
                Index2D newPosition = position.add(offset);
                if (get(newPosition) == get(position) + 1) {
                    out += getScore(newPosition, visited, round);
                }
            }
            visited.set(position, round);
            return out;
        }
    }

    int countAllPaths() {
        Grid<Integer> numPathsToTop = new Grid<Integer>(this.rows, this.cols);
        Integer paths = 0;
        for (int i = 1; i < this.rows - 1; i++) {
            for (int j = 1; j < this.cols - 1; j++) {
                Index2D position = new Index2D(i, j);
                if (get(position) == 0) {
                    paths += countPaths(position, numPathsToTop);
                }
            }
        }

        return paths;
    }

    int countPaths(Index2D position, Grid<Integer> numPathsToTop) {
        if (numPathsToTop.get(position) != null) {
            return numPathsToTop.get(position);
        } else if (get(position) == 9) {
            numPathsToTop.set(position, 1);
            return 1;
        } else {
            int paths = 0;
            for (Index2D offset : NEIGHBOR_OFFSETS) {
                Index2D newPosition = position.add(offset);
                if (get(newPosition) == get(position) + 1) {
                    paths += countPaths(newPosition, numPathsToTop);
                }
            }
            numPathsToTop.set(position, paths);
            return paths;
        }
    }
}
