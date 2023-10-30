#[derive(Copy, Clone)]
pub struct Cell {
    pub pos_x: u32,
    pub pos_y: u32,
    pub visited: bool,
    has_top_wall: bool,
    has_right_wall: bool,
    has_bottom_wall: bool,
    has_left_wall: bool,
}

impl Cell {
    pub fn new(pos_x: u32, pos_y: u32, visited: bool) -> Cell {
        Cell {
            pos_x,
            pos_y,
            visited,
            has_top_wall: true,
            has_right_wall: true,
            has_bottom_wall: true,
            has_left_wall: true,
        }
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn has_wall_top(&self) -> bool {
        self.has_top_wall
    }

    pub fn has_wall_right(&self) -> bool {
        self.has_right_wall
    }

    pub fn has_wall_bottom(&self) -> bool {
        self.has_bottom_wall
    }

    pub fn has_wall_left(&self) -> bool {
        self.has_left_wall
    }

    // Add functions to break walls
    pub fn break_wall_top(&mut self) {
        self.has_top_wall = false;
    }

    pub fn break_wall_right(&mut self) {
        self.has_right_wall = false;
    }

    pub fn break_wall_bottom(&mut self) {
        self.has_bottom_wall = false;
    }

    pub fn break_wall_left(&mut self) {
        self.has_left_wall = false;
    }
}
