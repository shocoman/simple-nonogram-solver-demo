use crate::board::{Board, Rule};

pub fn run_nonogram_solver() -> Board {
    let mut rules = vec![
        // rows
        Rule::new(0, false, vec![3, 7, 4]),
        Rule::new(1, false, vec![3, 2, 5, 1]),
        Rule::new(2, false, vec![4, 6, 4]),
        Rule::new(3, false, vec![4, 4, 4, 1]),
        Rule::new(4, false, vec![3, 4, 9]),
        Rule::new(5, false, vec![8, 10]),
        Rule::new(6, false, vec![6, 10]),
        Rule::new(7, false, vec![2, 2, 1, 6, 5]),
        Rule::new(8, false, vec![1, 2, 3, 9]),
        Rule::new(9, false, vec![3, 9, 2]),
        Rule::new(10, false, vec![4, 13]),
        Rule::new(11, false, vec![5, 6, 5, 1]),
        Rule::new(12, false, vec![2, 7, 2, 6]),
        Rule::new(13, false, vec![1, 17]),
        Rule::new(14, false, vec![1, 8, 3]),
        Rule::new(15, false, vec![9, 4]),
        Rule::new(16, false, vec![12]),
        Rule::new(17, false, vec![5, 6, 2]),
        Rule::new(18, false, vec![11]),
        Rule::new(19, false, vec![12, 3, 3]),
        // cols
        Rule::new(0, true, vec![1, 4, 8, 1]),
        Rule::new(1, true, vec![8, 4, 1]),
        Rule::new(2, true, vec![7, 4, 1]),
        Rule::new(3, true, vec![3, 4, 4, 1]),
        Rule::new(4, true, vec![3, 3, 1]),
        Rule::new(5, true, vec![1, 3, 2, 1, 1, 1]),
        Rule::new(6, true, vec![8, 3, 1, 1, 1]),
        Rule::new(7, true, vec![6, 1, 4, 1, 1, 1]),
        Rule::new(8, true, vec![1, 3, 13]),
        Rule::new(9, true, vec![4, 14]),
        Rule::new(10, true, vec![3, 3, 3, 4, 2]),
        Rule::new(11, true, vec![3, 16]),
        Rule::new(12, true, vec![1, 7, 7]),
        Rule::new(13, true, vec![2, 9, 7]),
        Rule::new(14, true, vec![1, 4, 7, 4]),
        Rule::new(15, true, vec![1, 17]),
        Rule::new(16, true, vec![1, 12, 4]),
        Rule::new(17, true, vec![1, 5, 7, 2]),
        Rule::new(18, true, vec![9, 8]),
        Rule::new(19, true, vec![2, 2, 8, 2, 1]),
    ];

    let rules2 = vec![
        // rows
        Rule::new(0, false, vec![5]),
        Rule::new(1, false, vec![1, 1]),
        Rule::new(2, false, vec![1, 1]),
        Rule::new(3, false, vec![1, 1]),
        Rule::new(4, false, vec![5]),
        Rule::new(5, false, vec![1, 1]),
        Rule::new(6, false, vec![7]),
        Rule::new(7, false, vec![2]),
        // cols
        Rule::new(0, true, vec![1]),
        Rule::new(1, true, vec![7]),
        Rule::new(2, true, vec![1, 1, 2]),
        Rule::new(3, true, vec![1, 1, 2]),
        Rule::new(4, true, vec![1, 1, 1]),
        Rule::new(5, true, vec![7]),
        Rule::new(6, true, vec![1]),
        Rule::new(7, true, vec![]),
    ];

    let rules_w = vec![
        // rows
        Rule::new(0, false, vec![8,7,5,7]),
        Rule::new(1, false, vec![5,4,3,3]),
        Rule::new(2, false, vec![3,3,2,3]),
        Rule::new(3, false, vec![4,3,2,2]),
        Rule::new(4, false, vec![3,3,2,2]),
        Rule::new(5, false, vec![3,4,2,2]),
        Rule::new(6, false, vec![4,5,2]),
        Rule::new(7, false, vec![3,5,1]),
        Rule::new(8, false, vec![4,3,2]),
        Rule::new(9, false, vec![3,4,2]),
        Rule::new(10, false, vec![4,4,2]),
        Rule::new(11, false, vec![3,6,2]),
        Rule::new(12, false, vec![3,2,3,1]),
        Rule::new(13, false, vec![4,3,4,2]),
        Rule::new(14, false, vec![3,2,3,2]),
        Rule::new(15, false, vec![6,5]),
        Rule::new(16, false, vec![4,5]),
        Rule::new(17, false, vec![3,3]),
        Rule::new(18, false, vec![3,3]),
        Rule::new(19, false, vec![1,1]),
        // cols
        Rule::new(0, true, vec![1]),
        Rule::new(1, true, vec![1]),
        Rule::new(2, true, vec![2]),
        Rule::new(3, true, vec![4]),
        Rule::new(4, true, vec![7]),
        Rule::new(5, true, vec![9]),
        Rule::new(6, true, vec![2,8]),
        Rule::new(7, true, vec![1,8]),
        Rule::new(8, true, vec![8]),
        Rule::new(9, true, vec![1,9]),
        Rule::new(10, true, vec![2,7]),
        Rule::new(11, true, vec![3,4]),
        Rule::new(12, true, vec![6,4]),
        Rule::new(13, true, vec![8,5]),
        Rule::new(14, true, vec![1,11]),
        Rule::new(15, true, vec![1,7]),
        Rule::new(16, true, vec![8]),
        Rule::new(17, true, vec![1,4,8]),
        Rule::new(18, true, vec![6,8]),
        Rule::new(19, true, vec![4,7]),
        Rule::new(20, true, vec![2,4]),
        Rule::new(21, true, vec![1,4]),
        Rule::new(22, true, vec![5]),
        Rule::new(23, true, vec![1,4]),
        Rule::new(24, true, vec![1,5]),
        Rule::new(25, true, vec![7]),
        Rule::new(26, true, vec![5]),
        Rule::new(27, true, vec![3]),
        Rule::new(28, true, vec![1]),
        Rule::new(29, true, vec![1]),
    ];

    // let board_size = (8, 8);
    // let board_size = (20, 20);
    let board_size = (20, 30);

    let mut board = Board::new(board_size.0, board_size.1, rules_w);
    // board.data[2][1] = Cell::On;
    // board.data[6][2] = Cell::Off;

    // board.solve();
    // board.print_board();

    return board;

    // println!("Rows: ");
    // for n_row in 0..board.rows {
    //     let row = board.get_row(n_row);
    //     print!("{}| ", n_row + 1);
    //     println!("{}", row.iter().map(|c| c.as_char()).collect::<String>());
    // }
    // println!();
    // println!("Cols: ");
    // for n_col in 0..board.cols {
    //     let mut col = board.get_col(n_col);
    //     *col[1] = Cell::On;
    //     print!("{}| ", n_col + 1);
    //     println!("{}", col.iter().map(|c| c.as_char()).collect::<String>());
    // }
}
