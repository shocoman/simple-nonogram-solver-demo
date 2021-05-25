
#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::board::{Board, Cell, Rule};

    fn row_to_str(row: &Vec<Cell>) -> String {
        row.iter().map(|c| c.as_char()).collect()
    }

    fn test_block_intersection(size: usize, hints: Vec<usize>, input: &str, correct_input: &str) {
        let mut row = if input.is_empty() {
            (0..size).map(|_| Cell::None).collect::<Vec<_>>()
        } else {
            input.chars().map(Cell::from_char).collect::<Vec<_>>()
        };

        let rule = Rule::new(0, false, hints);
        let mut ref_row = row.iter_mut().map(|c| c).collect::<Vec<_>>();

        Board::block_intersection(&rule, &mut ref_row);

        let correct_row = correct_input
            .chars()
            .map(Cell::from_char)
            .collect::<Vec<_>>();
        assert_eq!(row, correct_row);
    }

    fn test_simple_boxes(size: usize, hints: Vec<usize>, input: &str, correct_input: &str) {
        let mut row = if input.is_empty() {
            (0..size).map(|_| Cell::None).collect::<Vec<_>>()
        } else {
            input.chars().map(Cell::from_char).collect::<Vec<_>>()
        };

        let mut ref_row = row.iter_mut().map(|c| c).collect::<Vec<_>>();

        Board::simple_boxes(&hints, &mut ref_row);

        let correct_row = correct_input
            .chars()
            .map(Cell::from_char)
            .collect::<Vec<_>>();
        assert_eq!(row, correct_row);
    }

    // #[test]
    fn simple_block_intersection() {
        test_block_intersection(8, vec![4, 3], "        ", "XXXX.XXX");
        test_block_intersection(10, vec![8], "", "  XXXXXX  ");
        test_block_intersection(10, vec![4, 3], "", "  XX   X  ");

        // test_block_intersection(8, vec![3, 3], ".       ",".XXX.XXX");
        // test_block_intersection(8, vec![3, 3], ".X      ",".XXX.XXX");
        // test_block_intersection(8, vec![3, 3], ".XX     ",".XXX.XXX");
        // test_block_intersection(8, vec![3, 3], ".XXX    ",".XXX.XXX");
        // test_block_intersection(8, vec![3, 3], ".XXX.   ",".XXX.XXX");
        // test_block_intersection(8, vec![3, 3], ".XXX.X  ",".XXX.XXX");
        // test_block_intersection(8, vec![3, 3], ".XXX.XX ",".XXX.XXX");
        // test_block_intersection(8, vec![3, 3], ".XXX.XXX",".XXX.XXX");
        // test_block_intersection(8, vec![3, 3], ".XXX.  X",".XXX.XXX");
        // test_block_intersection(8, vec![3, 3], ".XXX. XX",".XXX.XXX");
        // test_block_intersection(8, vec![3, 3], ".XX . XX",".XXX.XXX");
        // test_block_intersection(8, vec![3, 3], " XXX XXX", ".XXX.XXX");
    }

    // #[test]
    fn simple_boxes() {
        test_simple_boxes(8, vec![4, 3], "        ", "XXXX XXX");
        test_simple_boxes(10, vec![8], "", "  XXXXXX  ");
        test_simple_boxes(10, vec![4, 3], "", "  XX   X  ");
        test_simple_boxes(10, vec![4, 3], ".        .", ".XXXX XXX.");
        test_simple_boxes(10, vec![4, 3], "        ..", "XXXX XXX..");
    }

    // #[test]
    fn find_first_cell_such_that() {
        let input = ".. X";

        let mut row = input.chars().map(Cell::from_char).collect::<Vec<_>>();

        let mut ref_row = row.iter_mut().map(|c| c).collect::<Vec<_>>();

        let fst_index = Board::find_first_cell_such_that(
            &mut ref_row,
            |cell| matches!(cell, Cell::On | Cell::None),
            false,
        );
        let lst_index = Board::find_first_cell_such_that(
            &mut ref_row,
            |cell| matches!(cell, Cell::On | Cell::None),
            true,
        );
        let lst_non_on = Board::find_first_cell_such_that(
            &mut ref_row,
            |cell| matches!(cell, Cell::On | Cell::None),
            true,
        );

        assert_eq!(fst_index, Some(2));
        assert_eq!(lst_index, Some(3));
        assert_eq!(lst_non_on, Some(3));
    }

    // #[test]
    fn fill_first_and_last_block_if_possible() {
        let mut row = ".X X  .XX X"
            .chars()
            .map(Cell::from_char)
            .collect::<Vec<_>>();
        let mut ref_row = row.iter_mut().map(|c| c).collect::<Vec<_>>();

        Board::fill_first_and_last_block_if_possible(&vec![1, 3, 4], &mut ref_row);

        assert_eq!(row_to_str(&row), ".X.XXX.XXXX");
    }

    // #[test]
    fn trim_finished_hints() {
        let mut row = ".XX..X XX.. X".chars().map(Cell::from_char).collect_vec();
        let mut ref_row = row.iter_mut().map(|c| c).collect_vec();

        let ((start, end), hints) = Board::trim_finished_hints(&vec![2, 4, 2], &mut ref_row);

        // assert!(hints.is_empty());
        assert_eq!(hints, vec![4, 2]);
        assert_eq!((start, end), (5, 12));
    }

    #[test]
    fn get_consecutive_regions() {
        let mut row = ".XX..X XX.. X".chars().map(Cell::from_char).collect_vec();
        let len = row.len();

        let mut ref_row = row.iter_mut().map(|c| c).collect_vec();

        let regions = Board::get_consecutive_regions(&mut ref_row, true, Some((3, len - 1)));

        // assert!(hints.is_empty());
        assert_eq!(regions, vec![(5, 4), (11, 2)]);
    }
}
