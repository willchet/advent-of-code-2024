public class Index2D {
    public int x;
    public int y;

    public Index2D(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public Index2D add(Index2D other) {
        return new Index2D(this.x + other.x, this.y + other.y);
    }

    public Index2D ne() {
        return new Index2D(this.x - 1, this.y + 1);
    }

    public Index2D se() {
        return new Index2D(this.x + 1, this.y + 1);
    }

    public Index2D sw() {
        return new Index2D(this.x + 1, this.y - 1);
    }

    public Index2D nw() {
        return new Index2D(this.x - 1, this.y - 1);
    }
}
