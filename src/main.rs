use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::thread;
use std::time::Duration;
use rand::prelude::SliceRandom;

mod backend;
use backend::cell::Cell;
use backend::maze::Maze;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;
const GRID_SIZE: usize = 16;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Maze Visualiser", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut stack = vec![(0, 0)];
    let mut maze = Maze::new();
    maze.visit(0, 0);

    while !stack.is_empty() {
        let (curr_x, curr_y) = *stack.last().unwrap();
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        let mut valid_directions = Vec::new();

        for &dir in directions.iter() {
            let (new_x, new_y) = (curr_x as i32 + dir.0, curr_y as i32 + dir.1);

            if new_x >= 0
                && new_x < GRID_SIZE as i32
                && new_y >= 0
                && new_y < GRID_SIZE as i32
                && !maze.maze[new_x as usize][new_y as usize].is_visited()
            {
                valid_directions.push(dir);
            }
        }

        if !valid_directions.is_empty() {
            let chosen_direction = *valid_directions.choose(&mut rand::thread_rng()).unwrap();
            let (new_x, new_y) = (curr_x as i32 + chosen_direction.0, curr_y as i32 + chosen_direction.1);

            maze.visit(new_x as usize, new_y as usize);
            maze.break_wall(curr_x, curr_y, chosen_direction);

            stack.push((new_x as usize, new_y as usize));
        } else {
            stack.pop();
        }

        thread::sleep(Duration::from_millis(50));

        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                let current_cell = maze.maze[x][y];
                let cell_color = if current_cell.is_visited() {
                    Color::RGB(0, 0, 240)
                } else {
                    Color::RGB(255, 255, 0)
                };

                let cell_rect = Rect::new(
                    (x as i32) * (SCREEN_WIDTH as i32 / GRID_SIZE as i32),
                    (y as i32) * (SCREEN_HEIGHT as i32 / GRID_SIZE as i32),
                    SCREEN_WIDTH / GRID_SIZE as u32,
                    SCREEN_HEIGHT / GRID_SIZE as u32,
                );

                canvas.set_draw_color(cell_color);
                canvas.fill_rect(cell_rect).unwrap();

                let wall_color = sdl2::pixels::Color::RGB(0, 0, 0);
                canvas.set_draw_color(wall_color);

                if current_cell.has_wall_top() {
                    let wall_rect = Rect::new(
                        (x as i32) * (SCREEN_WIDTH as i32 / GRID_SIZE as i32),
                        (y as i32) * (SCREEN_HEIGHT as i32 / GRID_SIZE as i32),
                        SCREEN_WIDTH / GRID_SIZE as u32,
                        1,
                    );
                    canvas.fill_rect(wall_rect).unwrap();
                }

                if current_cell.has_wall_right() {
                    let wall_rect = Rect::new(
                        ((x + 1) as i32) * (SCREEN_WIDTH as i32 / GRID_SIZE as i32) - 1,
                        (y as i32) * (SCREEN_HEIGHT as i32 / GRID_SIZE as i32),
                        1,
                        SCREEN_HEIGHT / GRID_SIZE as u32,
                    );
                    canvas.fill_rect(wall_rect).unwrap();
                }

                if current_cell.has_wall_bottom() {
                    let wall_rect = Rect::new(
                        (x as i32) * (SCREEN_WIDTH as i32 / GRID_SIZE as i32),
                        ((y + 1) as i32) * (SCREEN_HEIGHT as i32 / GRID_SIZE as i32) - 1,
                        SCREEN_WIDTH / GRID_SIZE as u32,
                        1,
                    );
                    canvas.fill_rect(wall_rect).unwrap();
                }

                if current_cell.has_wall_left() {
                    let wall_rect = Rect::new(
                        (x as i32) * (SCREEN_WIDTH as i32 / GRID_SIZE as i32),
                        (y as i32) * (SCREEN_HEIGHT as i32 / GRID_SIZE as i32),
                        1,
                        SCREEN_HEIGHT / GRID_SIZE as u32,
                    );
                    canvas.fill_rect(wall_rect).unwrap();
                }
            }
        }
        canvas.present();

    }
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
    }
}