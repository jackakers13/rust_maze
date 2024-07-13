mod maze;
use maze::Maze;

fn main() {
    let width = 25;
    let height = 25;
    let mut maze = Maze::new(width, height);
    maze.generate();
    maze.save_as_png("maze.png");
}
