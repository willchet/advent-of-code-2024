import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;

public class Grid<T> {
    static final Index2D[] NEIGHBOR_OFFSETS = new Index2D[] {
            new Index2D(-1, 0),
            new Index2D(0, -1),
            new Index2D(0, 1),
            new Index2D(1, 0),
    };
    static final Index2D[] NEIGHBOR_OFFSETS_DIAG = new Index2D[] {
            new Index2D(-1, -1),
            new Index2D(-1, 0),
            new Index2D(-1, 1),
            new Index2D(0, -1),
            new Index2D(0, 1),
            new Index2D(1, -1),
            new Index2D(1, 0),
            new Index2D(1, 1),
    };

    public T[][] grid;
    public int rows;
    public int cols;

    @SuppressWarnings("unchecked")
    public Grid(String filename, T padding) throws IOException {
        List<String> lines = Files.readAllLines(Paths.get(filename));

        int original_rows = lines.size();
        int original_cols = lines.get(0).length();
        this.rows = original_rows + 2;
        this.cols = original_cols + 2;

        this.grid = (T[][]) new Object[rows][cols];
        Arrays.fill(this.grid[0], padding);
        for (int i = 0; i < original_rows; i++) {
            this.grid[i + 1][0] = padding;
            for (int j = 0; j < original_cols; j++) {
                this.grid[i + 1][j + 1] = readChar(lines.get(i).charAt(j), padding);
            }
            this.grid[i + 1][cols - 1] = padding;
        }
        Arrays.fill(this.grid[rows - 1], padding);
    }

    @SuppressWarnings("unchecked")
    public Grid(T initial_value, int rows, int cols) {
        this.grid = (T[][]) new Object[rows][cols];
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                this.grid[i][j] = initial_value;
            }
        }
        this.rows = rows;
        this.cols = cols;
    }

    @SuppressWarnings("unchecked")
    public Grid(int rows, int cols) {
        this.grid = (T[][]) new Object[rows][cols];
        this.rows = rows;
        this.cols = cols;
    }

    @SuppressWarnings("unchecked")
    private T readChar(char c, T padding) {
        if (Character.class.isAssignableFrom(padding.getClass())) {
            return (T) Character.valueOf(c);
        } else if (Integer.class.isAssignableFrom(padding.getClass())) {
            return (T) Integer.valueOf(c - '0');
        } else if (Byte.class.isAssignableFrom(padding.getClass())) {
            return (T) Byte.valueOf((byte) (c - '0'));
        } else {
            throw new IllegalArgumentException("Unsupported type for padding: " + padding.getClass());
        }
    }

    public T get(Index2D index) {
        return this.grid[index.x][index.y];
    }

    public void set(Index2D index, T value) {
        this.grid[index.x][index.y] = value;
    }
}
