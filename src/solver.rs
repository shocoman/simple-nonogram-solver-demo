use crate::board::{Board, Cell, Rule};

pub fn solve(board: &mut Board, clear_board: bool) -> bool {
    if clear_board {
        board.clear_board();
    }

    let mut rules = board.rules.clone();

    loop {
        let previous_board = board.data.clone();
        for rule in rules.iter() {
            let mut col_or_row = if rule.is_col {
                board.get_col(rule.n)
            } else {
                board.get_row(rule.n)
            };

            // mark overlapping blocks
            simple_boxes(&rule.hints, &mut col_or_row);

            // // fill best (if only one) region for a hint
            // Board::fill_only_one_possible_hint_region(rule, &mut col_or_row);

            // if all hints are complete, mark other cells as OFF
            mark_complete_row(rule, &mut col_or_row);

            fill_first_and_last_block_if_possible(&rule.hints, &mut col_or_row);

            // // enclose longest completed region with OFF cells
            // Board::enclose_completed_region(rule, &mut col_or_row);

            // if region is smaller that any hint, mark it as OFF
            cross_impossible_region(&rule.hints, &mut col_or_row);

            if rule.hints.is_empty() {
                col_or_row.iter_mut().for_each(|cell| **cell = Cell::Off);
            }
        }

        if previous_board == board.data {
            break;
        }
    }

    // board.print_board();

    return is_solved(board);
}

pub fn is_solved(board: &Board) -> bool {
    for cell in board.data.iter().flatten() {
        if let Cell::None = cell {
            return false;
        }
    }
    true
}

pub fn mark_complete_row(rule: &Rule, col_or_row: &mut Vec<&mut Cell>) {
    let row_is_complete = rule.hints.iter().copied().sum::<usize>()
        == col_or_row.iter().filter(|c| matches!(c, Cell::On)).count();
    if row_is_complete {
        for cell in col_or_row.iter_mut() {
            if let Cell::None = cell {
                **cell = Cell::Off;
            }
        }
    }
}

pub fn enclose_completed_region(rule: &Rule, col_or_row: &mut Vec<&mut Cell>) {
    // check region at row start and row end
    // if Some(&&mut Cell::On) = col_or_row.first() {
    //
    // }

    // check maximum hint region
    if let Some(max_hint) = rule.hints.iter().copied().max() {
        // find max consecutive region of size 'max_hint'
        let consecutive_on_cells = Board::get_consecutive_regions(&col_or_row, false, None);
        if let Some(&(reg_start, reg_len)) = consecutive_on_cells.iter().find(|&&r| r.1 == max_hint)
        {
            let reg_end = reg_start + reg_len - 1;
            if reg_start > 0 {
                *col_or_row[reg_start - 1] = Cell::Off;
            }
            if reg_end < col_or_row.len() - 1 {
                *col_or_row[reg_end + 1] = Cell::Off;
            }
        }
    }
}

pub fn cross_impossible_region(hints: &Vec<usize>, row: &mut Vec<&mut Cell>) {
    let ((start, end), hints) = trim_finished_hints(hints, row);
    if hints.is_empty() {
        return;
    }

    let mut regions =
        Board::get_consecutive_regions(&row, true, Some((start as usize, end as usize)));

    if let Some(&smallest_hint) = hints.iter().min() {
        for &(reg_pos, reg_len) in regions.iter() {
            if reg_len < smallest_hint {
                for i in reg_pos..reg_pos + reg_len {
                    *row[i] = Cell::Off;
                }
            }
        }
    }
}

pub fn fill_only_one_possible_hint_region(rule: &Rule, mut col_or_row: &mut Vec<&mut Cell>) {
    let mut open_regions = Board::get_consecutive_regions(&col_or_row, true, None);

    for &hint in rule.hints.iter() {
        let good_regions = open_regions
            .iter()
            .filter(|&&region| hint <= region.1)
            .collect::<Vec<_>>();
        if good_regions.len() != 1 {
            break;
        }
        let good_region = *good_regions[0];
        let skip = good_region.1 - hint;
        for i in skip..good_region.1 - skip {
            *col_or_row[good_region.0 + i] = Cell::On;
        }

        // fill holes in region
        let mut region_indices = good_region.0..good_region.0 + good_region.1;
        let first_on_cell = region_indices.find(|&i| col_or_row[i] == &Cell::On);
        let last_on_cell = region_indices.rfind(|&i| col_or_row[i] == &Cell::On);
        if let (Some(fst), Some(lst)) = (first_on_cell, last_on_cell) {
            if hint >= lst - fst {
                for i in fst..=lst {
                    *col_or_row[i] = Cell::On;
                }
            }
        }
    }
}

pub fn trim_finished_hints(
    hints: &Vec<usize>, row: &mut Vec<&mut Cell>,
) -> ((i32, i32), Vec<usize>) {
    let mut hints = hints.clone();
    let (mut start_pos, mut end_pos) = (0, row.len() as i32 - 1);

    // find zone starting position
    let mut i = 0_i32;
    while i < row.len() as i32 {
        if *row[i as usize] == Cell::Off {
            start_pos = i + 1;
            i += 1;
        } else if *row[i as usize] == Cell::On {
            let block_length = Board::count_block_length(row, i as usize, false);
            if block_length == hints[0] {
                hints.remove(0);
                if hints.is_empty() {
                    break;
                }

                i += block_length as i32;
                *row[i as usize] = Cell::Off;
                start_pos = i + 1;
            }
            i += 1;
        } else {
            break;
        }
    }

    // find zone ending position
    i = row.len() as i32 - 1;
    while i >= 0 {
        if hints.is_empty() {
            break;
        }

        if *row[i as usize] == Cell::Off {
            end_pos = i - 1;
            i -= 1;
        } else if *row[i as usize] == Cell::On {
            let block_length = Board::count_block_length(row, i as usize, true);
            if block_length == hints[hints.len() - 1] {
                hints.pop();
                i -= block_length as i32;
                if i >= 0 {
                    *row[i as usize] = Cell::Off;
                    end_pos = i - 1;
                }
            }
            i -= 1;
        } else {
            break;
        }
    }

    // println!( "Start pos: {}; End pos: {}; Hints: {:?}",  start_pos, end_pos, hints);
    // if hints.is_empty() {
    //     start_pos = row.len() as i32 - 1;
    //     end_pos = -1;
    // }

    if hints.is_empty() {
        start_pos = row.len() as i32;
        end_pos = -1;
    }

    ((start_pos, end_pos), hints)
}

pub fn block_intersection(rule: &Rule, row: &mut Vec<&mut Cell>) {
    let mut hints = rule.hints.clone();
    let (mut start_pos, mut end_pos) = (0, row.len() - 1);

    println!("Row: {:?}", row);
    let mut i = 0;
    while i < row.len() {
        if *row[i] == Cell::Off {
            start_pos = i + 1;
            i += 1;
        } else if *row[i] == Cell::On {
            let block_length = Board::count_block_length(row, i, false);
            if block_length == *hints.first().unwrap() {
                hints.remove(0);
                i += block_length;
                if i < row.len() {
                    *row[i] = Cell::Off;
                    start_pos = i + 1;
                }
            }
            i += 1;
        } else {
            break;
        }
    }
    if i >= row.len() {
        return;
    }

    let mut i = row.len() - 1;
    while i >= 0 {
        if *row[i] == Cell::Off {
            end_pos = i - 1;
            i -= 1;
        } else if *row[i] == Cell::On {
            let block_length = Board::count_block_length(row, i, true);
            if hints.is_empty() {
                return;
            }

            if block_length == *hints.last().unwrap() {
                hints.pop();
                i -= block_length;
                if i >= 0 {
                    *row[i] = Cell::Off;
                    end_pos = i - 1;
                }
            }
            i -= 1;
        } else {
            break;
        }
    }

    println!("Start pos: {}; End pos: {}", start_pos, end_pos);

    // find forward and backward blocks with hints
    let mut forward_blocks = vec![];
    let mut pos = start_pos as i32;
    for hint in hints.iter() {
        forward_blocks.push((pos, pos + *hint as i32 - 1));
        pos += *hint as i32 + 1;
    }

    let mut reverse_blocks = vec![];
    let mut pos = end_pos as i32;
    for hint in hints.iter().rev() {
        reverse_blocks.insert(0, (pos - *hint as i32 + 1, pos));
        pos -= *hint as i32 + 1;
    }

    // find intersection
    let mut block_intersections = vec![];
    for (b1, b2) in forward_blocks.iter().zip(reverse_blocks.iter()) {
        let (&(b1_start, b1_end), &(b2_start, b2_end)) = (b1, b2);
        let start = b1_start.max(b2_start);
        let end = b1_end.min(b2_end);
        if start <= end {
            block_intersections.push((start, end));
        }
    }

    // mark intersections
    for (i, &(start, end)) in block_intersections.iter().enumerate() {
        for j in start as usize..=end as usize {
            *row[j] = Cell::On;
        }
        // if intersection is perfect, surround block with crosses
        if (end + 1 - start) == hints[i] as i32 {
            row.get_mut((start - 1) as usize)
                .and_then(|cell| Some(**cell = Cell::Off));
            row.get_mut((end + 1) as usize)
                .and_then(|cell| Some(**cell = Cell::Off));
        }
    }
}

pub fn simple_boxes(hints: &Vec<usize>, row: &mut Vec<&mut Cell>) {
    let ((start, end), hints) = trim_finished_hints(&hints, row);

    // find forward and backward blocks with hints
    let mut forward_blocks = vec![];
    let mut pos = start;
    for hint in hints.iter() {
        forward_blocks.push((pos, pos + *hint as i32 - 1));
        pos += *hint as i32 + 1;
    }

    let mut reverse_blocks = vec![];
    let mut pos = end;
    for hint in hints.iter().rev() {
        reverse_blocks.insert(0, (pos - *hint as i32 + 1, pos));
        pos -= *hint as i32 + 1;
    }

    // find intersection
    let mut block_intersections = vec![];
    for (b1, b2) in forward_blocks.iter().zip(reverse_blocks.iter()) {
        let (&(b1_start, b1_end), &(b2_start, b2_end)) = (b1, b2);
        let start = b1_start.max(b2_start);
        let end = b1_end.min(b2_end);
        if start <= end {
            block_intersections.push((start, end));
        }
    }

    // mark intersections
    for &(start, end) in block_intersections.iter() {
        for i in start as usize..=end as usize {
            *row[i] = Cell::On;
        }
    }
}

pub fn fill_first_and_last_block_if_possible(hints: &Vec<usize>, row: &mut Vec<&mut Cell>) {
    let ((start, end), hints) = trim_finished_hints(&hints, row);
    if hints.is_empty() {
        return;
    }

    let (fst_index, lst_index) = (start as usize, end as usize);
    let (fst_cell, lst_cell): (Cell, Cell) = (row[fst_index].clone(), row[lst_index].clone());
    if let Cell::On = fst_cell {
        for i in 0..hints[0] {
            *row[fst_index + i] = Cell::On;
        }
        row.get_mut(fst_index + hints[0])
            .and_then(|cell| Some(**cell = Cell::Off));
    }
    if let Cell::On = lst_cell {
        for i in 0..hints[hints.len() - 1] {
            *row[lst_index - i] = Cell::On;
        }
        row.get_mut(lst_index - hints[hints.len() - 1])
            .and_then(|cell| Some(**cell = Cell::Off));
    }
}
