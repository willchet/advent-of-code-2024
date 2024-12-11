import java.io.IOException;

public class Day4 extends Grid<Character> {
    Day4(String filename) throws IOException {
        super(filename, '.');
    }

    public static void main(String[] args) {
        if (args.length > 0) {
            String filename = args[0];
            try {
                Day4 grid = new Day4(filename);
                int wordCount = grid.countWords();
                int crossCount = grid.countCrosses();
                System.out.println(wordCount + ", " + crossCount);
            } catch (IOException e) {
                e.printStackTrace();
            }
        } else {
            System.out.println("No filename provided.");
        }
    }

    Boolean checkForWord(Index2D position, Index2D offset) {
        position = position.add(offset);
        for (char c : "MAS".toCharArray()) {
            if (get(position) == c) {
                position = position.add(offset);
            } else {
                return false;
            }
        }
        return true;
    }

    int countWordsAtSpot(Index2D position) {
        int count = 0;
        if (get(position) == 'X') {
            for (Index2D offset : NEIGHBOR_OFFSETS_DIAG) {
                if (checkForWord(position, offset)) {
                    count += 1;
                }
            }
        }
        return count;
    }

    int countWords() {
        int count = 0;
        for (int i = 1; i < rows - 1; i++) {
            for (int j = 1; j < cols - 1; j++) {
                count += countWordsAtSpot(new Index2D(i, j));
            }
        }
        return count;
    }

    Boolean checkForCross(Index2D position) {
        return get(position) == 'A'
                && (get(position.nw()) == 'M' && get(position.se()) == 'S'
                        || get(position.nw()) == 'S' && get(position.se()) == 'M')
                && (get(position.ne()) == 'M' && get(position.sw()) == 'S'
                        || get(position.ne()) == 'S' && get(position.sw()) == 'M');
    }

    int countCrosses() {
        int count = 0;
        for (int i = 1; i < rows - 1; i++) {
            for (int j = 1; j < cols - 1; j++) {
                if (checkForCross(new Index2D(i, j))) {
                    count++;
                }
            }
        }
        return count;
    }
}
