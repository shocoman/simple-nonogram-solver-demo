use macroquad::color::{BLUE, GRAY, GREEN, SKYBLUE};
use macroquad::math::{bool, f32, i32};
use macroquad::prelude::{draw_line, draw_rectangle, draw_text, screen_height, screen_width};
use itertools::Itertools;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    None, // uncertain (maybe painted or not)
    On,   // cell should be painted
    Off,  // cell should be empty
}

impl Cell {
    pub fn as_char(&self) -> char {
        match self {
            Cell::On => 'X',
            Cell::Off => '.',
            Cell::None => ' ',
        }
    }

    pub fn from_char(ch: char) -> Self {
        match ch {
            _ if ch == Self::as_char(&Self::On) => Self::On,
            _ if ch == Self::as_char(&Self::Off) => Self::Off,
            _ => Self::None,
        }
    }

    pub fn flip(&self) -> Self {
        match self {
            Cell::None => Cell::On,
            Cell::On => Cell::None,
            Cell::Off => Cell::On,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub n: usize,
    pub is_col: bool,
    pub hints: Vec<usize>,
}

impl Rule {
    pub fn new(n: usize, is_col: bool, hints: Vec<usize>) -> Self {
        Rule { n, is_col, hints }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<Cell>>,
    pub rules: Vec<Rule>,
}

impl Board {
    pub fn new(rows: usize, cols: usize, rules: Vec<Rule>) -> Self {
        Board {
            rows,
            cols,
            data: vec![vec![Cell::None; cols]; rows],
            rules,
        }
    }

    pub fn get_row(&mut self, n: usize) -> Vec<&mut Cell> {
        self.data.get_mut(n).unwrap().iter_mut().collect()
    }

    pub fn get_col(&mut self, n: usize) -> Vec<&mut Cell> {
        self.data
            .iter_mut()
            .map(|row| row.get_mut(n).unwrap())
            .collect()
    }

    pub fn boards_are_equal(&self, board: &Board) -> bool {
        if self.rows != board.rows || self.cols != board.cols { return false; }

        for n_row in 0..self.rows {
            for n_col in 0..self.rows {
                if self.data[n_row][n_col] == Cell::On && board.data[n_row][n_col] != Cell::On {
                    return false
                }
                if board.data[n_row][n_col] == Cell::On && self.data[n_row][n_col] != Cell::On {
                    return false
                }
            }
        }

        true
    }

    pub fn print_board(&self) {
        let max_hints = [|r: &Rule| !r.is_col, |r: &Rule| r.is_col]
            .iter()
            .map(|f| {
                self.rules
                    .iter()
                    .filter_map(|r| f(r).then(|| r.hints.len()))
                    .max()
            })
            .flatten()
            .collect_vec();

        let (max_row_hints, max_col_hints) = (max_hints[0], max_hints[1]);
        dbg!(max_row_hints, max_col_hints);

        let row_hints = self.rules.iter().filter(|r| !r.is_col).collect::<Vec<_>>();
        let col_hints = self.rules.iter().filter(|r| r.is_col).collect::<Vec<_>>();

        // print col hints
        let mut str_rows: Vec<String> = vec![];
        let row_padding = max_row_hints + 1;
        for row in 0..max_col_hints {
            let mut s = String::with_capacity(row_padding);
            for _ in 0..row_padding {
                s.push(' ');
            }
            for col in 0..self.cols {
                if let Some(hint) = col_hints[col].hints.get(row) {
                    s.push(hint.to_string().chars().next().unwrap());
                } else {
                    s.push(' ');
                }
            }
            str_rows.push(s);
        }
        let mut row_edge = String::new();
        for row in 0..self.cols + row_padding {
            row_edge.push(if row < row_padding { ' ' } else { '_' });
        }
        str_rows.push(row_edge);

        // print board and row hints
        for row in 0..self.rows {
            let mut s = String::with_capacity(row_padding);
            for col in 0..max_row_hints {
                if let Some(hint) = row_hints[row].hints.get(col) {
                    s.push(hint.to_string().chars().next().unwrap());
                } else {
                    s.push(' ');
                }
            }

            s.push('|');

            for cell in self.data[row].iter() {
                s.push(cell.as_char());
            }

            str_rows.push(s);
        }

        for row in str_rows.iter() {
            println!("{}", row);
        }
    }

    pub fn get_consecutive_regions(
        row: &Vec<&mut Cell>, include_none_cells: bool, incl_interval: Option<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let (start, end) = incl_interval.unwrap_or((0, row.len() - 1));

        let (mut pos, mut length) = (0, 0);

        let mut regions = vec![];
        for i in start..=end {
            let cell = &row[i];
            if cell == &&Cell::On || (include_none_cells && cell == &&Cell::None) {
                if length == 0 {
                    pos = i;
                }
                length += 1;
            } else if length > 0 {
                regions.push((pos, length));
                length = 0;
            }
        }
        if length > 0 {
            regions.push((pos, length));
        }

        regions
    }

    pub fn count_block_length(row: &mut Vec<&mut Cell>, pos: usize, count_backward: bool) -> usize {
        let mut length = 0;
        if count_backward {
            while let Some(Cell::On) = row.get(pos - length) {
                length += 1;
            }
        } else {
            while let Some(Cell::On) = row.get(pos + length) {
                length += 1;
            }
        }

        return length;
    }

    pub fn find_first_cell_such_that(
        row: &mut Vec<&mut Cell>, f: fn(&Cell) -> bool, reverse: bool,
    ) -> Option<usize> {
        if reverse {
            for i in (0..row.len()).rev() {
                if f(row[i]) {
                    return Some(i);
                }
            }
        } else {
            for i in 0..row.len() {
                if f(row[i]) {
                    return Some(i);
                }
            }
        }

        None
    }

    pub fn clear_board(&mut self) {
        self.data = vec![vec![Cell::None; self.cols]; self.rows];
    }

    pub fn generate_new_rules_according_to_board(&mut self) -> Vec<Rule> {
        let mut rules = vec![];

        // rows
        for n_row in 0..self.rows {
            let row = self.get_row(n_row);
            let regions = Self::get_consecutive_regions(&row, false, None);
            let rule = Rule::new(
                n_row,
                false,
                regions.iter().map(|&r| r.1).collect(),
            );
            rules.push(rule);
        }

        // cols
        for n_col in 0..self.rows {
            let row = self.get_col(n_col);
            let regions = Self::get_consecutive_regions(&row, false, None);
            let rule = Rule::new(n_col, true, regions.iter().map(|&r| r.1).collect());
            rules.push(rule);
        }

        rules
    }

    pub fn draw_board(&self) {
        let row_hints = self.rules.iter().filter(|r| !r.is_col).collect::<Vec<_>>();
        let col_hints = self.rules.iter().filter(|r| r.is_col).collect::<Vec<_>>();

        let max_hints = [&row_hints, &col_hints]
            .iter()
            .map(|hints| hints.iter().map(|r| r.hints.len()).max())
            .flatten()
            .collect::<Vec<_>>();
        let (max_row_hints, max_col_hints) = (max_hints[0], max_hints[1]);

        // print col hints
        let mut str_rows: Vec<String> = vec![];
        let row_padding = max_row_hints;
        for row in 0..max_col_hints {
            let mut s = String::with_capacity(row_padding);
            for _ in 0..row_padding {
                s.push(' ');
            }
            for col in 0..self.cols {
                if let Some(hint) = col_hints[col].hints.get(row) {
                    s.push(*hint as u8 as char);
                } else {
                    s.push(' ');
                }
            }
            str_rows.push(s);
        }

        // print board and row hints
        for row in 0..self.rows {
            let mut s = String::with_capacity(row_padding);
            for col in 0..max_row_hints {
                if let Some(hint) = row_hints[row].hints.get(col) {
                    s.push(*hint as u8 as char);
                } else {
                    s.push(' ');
                }
            }

            for cell in self.data[row].iter() {
                s.push(cell.as_char());
            }

            str_rows.push(s);
        }

        let screen_size = screen_width().min(screen_height());
        let rect_size = screen_size / str_rows.len() as f32;

        for (n_row, row) in str_rows.iter().enumerate() {
            for (n_col, ch) in row.char_indices() {
                let size_offset = 0.05 * rect_size;
                let x = rect_size * n_col as f32 + size_offset;
                let y = rect_size * n_row as f32 + size_offset;
                let text_y = rect_size * (n_row as f32 + 0.5) + size_offset;
                let size = rect_size - size_offset;
                if ch == Cell::On.as_char() {
                    draw_rectangle(x, y, size, size, GREEN);
                } else if ch == Cell::Off.as_char() {
                    draw_rectangle(x, y, size, size, GRAY);
                } else if ch == Cell::None.as_char()
                    && n_row >= max_col_hints
                    && n_col >= max_row_hints
                {
                    // draw_rectangle(x, y, size, size, DARKGRAY);
                } else if (ch as u8) < 30 {
                    draw_text(&(ch as u8).to_string(), x, text_y, size, BLUE);
                }
            }
        }

        // draw border
        let line_x = max_row_hints as f32 * rect_size;
        let line_y = max_col_hints as f32 * rect_size + 1.5;
        let line_x_end = (max_row_hints + self.cols) as f32 * rect_size;
        let line_thickness = 3.0;
        draw_line(
            line_x,
            line_y,
            line_x,
            screen_height(),
            line_thickness,
            SKYBLUE,
        ); // left edge
        draw_line(line_x, line_y, line_x_end, line_y, line_thickness, SKYBLUE); // top edge
        draw_line(
            line_x_end,
            line_y,
            line_x_end,
            screen_height(),
            line_thickness,
            SKYBLUE,
        );
        // right edge
    }
}
