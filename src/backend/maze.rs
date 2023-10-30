use crate::Cell;
pub struct Maze {
    pub maze: Vec<Vec<Cell>>,
    pub curr_pos_x: u32,
    pub curr_pos_y: u32,
}

impl Maze {
    pub fn new() -> Maze {
        let mut maze = vec![];
        for _ in 0..16 {
            let mut row = vec![];
            for _ in 0..16 {
                row.push(Cell::new(0, 0, false));
            }
            maze.push(row);
        }
        Maze {
            maze,
            curr_pos_x: 0,
            curr_pos_y: 0,
        }
    }

    pub fn visit(&mut self, x: usize, y: usize) {
        if x < self.maze.len() && y < self.maze[0].len() {
            if !self.maze[x][y].visited {
                self.maze[x][y].visited = true;
            }
        } else {
            println!("Error: Cell coordinates out of bounds");
        }
    }

    pub fn break_wall(&mut self, x: usize, y: usize, direction: (i32, i32)) {
        let (dx, dy) = direction;
        if dx == 1 {
            self.maze[x][y].break_wall_right();
            self.maze[x + 1][y].break_wall_left();
        } else if dx == -1 {
            self.maze[x][y].break_wall_left();
            self.maze[x - 1][y].break_wall_right();
        } else if dy == 1 {
            self.maze[x][y].break_wall_bottom();
            self.maze[x][y + 1].break_wall_top();
        } else if dy == -1 {
            self.maze[x][y].break_wall_top();
            self.maze[x][y - 1].break_wall_bottom();
        }
    }
}
