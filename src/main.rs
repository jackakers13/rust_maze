mod maze;
use maze::Maze;

fn main() {
    let width = 384;
    let height = 128;
    let mut maze = Maze::new(width, height);
    maze.generate();
    maze.save_as_png("maze.png", None);

    if let Some(solution) = maze.solve() {
        maze.save_as_png("maze-solved.png", Some(&solution));
    } else {
        println!("No solution found for the maze.");
    }
}
