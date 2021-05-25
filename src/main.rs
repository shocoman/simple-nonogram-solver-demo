use std::io::Read;

use itertools::Itertools;
use macroquad::prelude::*;

use crate::board::{Board, Cell, Rule};

mod board;
mod nonogram_solver;
mod solver;
mod tests;

#[derive(Debug, Eq, PartialEq)]
pub enum Mode {
    Create,
    Play,
}

struct Nonogram {
    create_board: Board,
    play_board: Board,
    is_solvable: bool,
    is_solved: bool,
    mode: Mode,
}

impl<'board> Nonogram {
    pub fn new(rows: usize, cols: usize) -> Self {
        Nonogram {
            create_board: Board::new(rows, cols, vec![]),
            play_board: Board::new(rows, cols, vec![]),
            is_solvable: false,
            mode: Mode::Create,
            is_solved: false,
        }
    }

    pub fn play_with_board(board: Board) -> Self {
        Nonogram {
            create_board: Board::new(board.rows, board.cols, vec![]),
            play_board: board,
            is_solvable: false,
            mode: Mode::Play,
            is_solved: false,
        }
    }

    pub fn change_mode(&mut self) {
        // let mut board_copy = self.board.clone();
        // solver::solve(&mut board_copy, true);
        // *self.board = board_copy;
        self.mode = match self.mode {
            Mode::Create => {
                self.play_board.clear_board();
                self.play_board.rules = self.create_board.rules.clone();

                Mode::Play
            }
            Mode::Play => Mode::Create,
        }

    }

    pub fn update(&mut self) {
        let active_board = match self.mode {
            Mode::Play => &mut self.play_board,
            Mode::Create => &mut self.create_board,
        };

        let screen_size = screen_width().min(screen_height()) * 0.65f32;
        let dimension = active_board.rows.max(active_board.cols);
        let rect_size = screen_size / dimension as f32;

        let start_x = screen_width() - active_board.cols as f32 * rect_size;
        let start_y = screen_height() - active_board.rows as f32 * rect_size;

        if is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_pressed(MouseButton::Right)
        {
            let (mouse_x, mouse_y) = mouse_position();
            let cell_x = (mouse_x - start_x) / rect_size;
            let cell_y = (mouse_y - start_y) / rect_size;
            let n_row = cell_y as usize;
            let n_col = cell_x as usize;

            let clicked = active_board
                .data
                .get_mut(n_row)
                .and_then(|row| row.get_mut(n_col))
                .and_then(|mut cell| {
                    let res = if is_mouse_button_pressed(MouseButton::Right) {
                        *cell = Cell::Off
                    } else {
                        *cell = cell.flip()
                    };
                    Some(res)
                });
            if clicked.is_some() {
                if self.mode == Mode::Create {
                    active_board.rules = active_board.generate_new_rules_according_to_board();
                }

                let mut board_copy = active_board.clone();
                self.is_solvable = solver::solve(&mut board_copy, true);

                if self.mode == Mode::Play {
                    self.is_solved = active_board.boards_are_equal(&board_copy);
                }
            }
        } else if is_key_pressed(KeyCode::S) {
            let mut board_copy = active_board.clone();
            solver::solve(&mut board_copy, true);
            *active_board = board_copy;
        } else if is_key_pressed(KeyCode::Space) {
            self.change_mode();
        }
    }

    pub fn draw(&mut self) {
        let active_board = match self.mode {
            Mode::Play => &mut self.play_board,
            Mode::Create => &mut self.create_board,
        };

        let screen_size = screen_width().min(screen_height()) * 0.65f32;
        let dimension = active_board.rows.max(active_board.cols);
        let rect_size = screen_size / dimension as f32;

        let start_x = screen_width() - active_board.cols as f32 * rect_size;
        let start_y = screen_height() - active_board.rows as f32 * rect_size;

        // write whether it is solvable or not
        let solvable_text = if self.is_solvable {
            "Board is solvable"
        } else {
            "Board isn't solvable"
        };
        let text_size = rect_size * 2f32;
        let text_dimensions = measure_text(solvable_text, None, text_size as u16, 1.0f32);
        draw_text(
            solvable_text,
            0_f32,
            text_dimensions.height + text_size,
            text_size,
            WHITE,
        );
        let mode_str = format!("Mode: {:?}; Solved?: {:?}", self.mode, self.is_solved);
        draw_text(
            mode_str.as_str(),
            0_f32,
            text_dimensions.height,
            text_size,
            WHITE,
        );

        // draw board
        for (n_row, row) in active_board.data.iter().enumerate() {
            for (n_col, cell) in row.iter().enumerate() {
                let size = rect_size;
                let (x_offset, y_offset) = (n_col as f32 * size, n_row as f32 * size);
                let (x, y) = (start_x + x_offset, start_y + y_offset);

                draw_rectangle_lines(x, y, size, size, 1f32, GRAY);

                match cell {
                    Cell::None => {}
                    Cell::On => draw_rectangle(x, y, size, size, GREEN),
                    Cell::Off => {
                        // draw_rectangle(x, y, size, size, GRAY)
                        draw_line(x, y, x + rect_size, y + rect_size, 1f32, GRAY);
                        draw_line(x + rect_size, y, x, y + rect_size, 1f32, GRAY);
                    }
                }
            }
        }

        // draw edges
        let line_thickness = 2.0f32;
        draw_line(
            start_x,
            start_y,
            screen_width(),
            start_y,
            line_thickness,
            SKYBLUE,
        ); // left edge
        draw_line(
            start_x,
            start_y,
            start_x,
            screen_height(),
            line_thickness,
            SKYBLUE,
        ); // top edge

        // draw rules
        for rule in active_board.rules.iter() {
            for (i, num) in rule.hints.iter().rev().enumerate() {
                let (x, y) = if rule.is_col {
                    (
                        start_x + rect_size * rule.n as f32,
                        start_y - rect_size - i as f32 * rect_size,
                    )
                } else {
                    (
                        start_x - rect_size - i as f32 * rect_size,
                        start_y + rect_size * rule.n as f32,
                    )
                };

                let num_str = num.to_string();

                let text_dimensions = measure_text(num_str.as_str(), None, rect_size as u16, 1f32);
                let TextDimensions {
                    width,
                    height,
                    offset_y,
                } = text_dimensions;

                let text_x_offset = (rect_size - width) / 2f32;
                let text_y_offset = (rect_size - height) / 2f32;

                draw_rectangle_lines(x, y, rect_size, rect_size, 0.5f32, WHITE);
                draw_text(
                    num_str.as_str(),
                    x + text_x_offset,
                    y + rect_size - text_y_offset,
                    rect_size,
                    BLUE,
                );
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        // fullscreen: true,
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board = nonogram_solver::run_nonogram_solver();
    // // solver::solve(&mut board);
    let mut nonogram = Nonogram::play_with_board(board);

    // let mut board = Board::new(8, 8, vec![]);

    // let mut nonogram = Nonogram::new(8, 8);



    loop {
        clear_background(BLACK);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle_lines(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, 10.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //
        // draw_text("1!", 20.0, 20.0, 30.0, DARKGRAY);

        nonogram.update();
        nonogram.draw();

        next_frame().await
    }
}
