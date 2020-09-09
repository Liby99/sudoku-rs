use sudoku_rs::*;

#[test]
fn solve_9x9() {
  let board = Board9x9::default();
  let mut solutions = board.solve(true);
  println!("{}", solutions.next().unwrap().to_string());
  println!("{}", solutions.next().unwrap().to_string());
  println!("{}", solutions.next().unwrap().to_string());
  println!("{}", solutions.next().unwrap().to_string());
}

#[test]
fn solve_pre_filled_9x9() {
  // let board = sudoku9x9![
  //   5, 3, 0, 0, 7, 0, 0, 0, 0;
  //   6, 0, 0, 1, 9, 5, 0, 0, 0;
  //   0, 9, 8, 0, 0, 0, 0, 6, 0;
  //   8, 0, 0, 0, 6, 0, 0, 0, 3;
  //   4, 0, 0, 8, 0, 3, 0, 0, 1;
  //   7, 0, 0, 0, 2, 0, 0, 0, 6;
  //   0, 6, 0, 0, 0, 0, 2, 8, 0;
  //   0, 0, 0, 4, 1, 9, 0, 0, 5;
  //   0, 0, 0, 0, 8, 0, 0, 7, 9;
  // ];
  let board = sudoku9x9![
    9, 0, 0, 0, 0, 2, 0, 0, 6;
    0, 8, 0, 0, 0, 7, 0, 5, 0;
    0, 0, 7, 0, 0, 8, 1, 0, 0;
    0, 0, 0, 6, 0, 0, 7, 1, 2;
    0, 0, 0, 0, 5, 0, 0, 0, 0;
    1, 2, 3, 0, 0, 4, 0, 0, 0;
    0, 0, 4, 8, 0, 0, 3, 0, 0;
    0, 3, 0, 7, 0, 0, 0, 2, 0;
    5, 0, 0, 4, 0, 0, 0, 0, 1;
  ];
  for solution in board.solve(true) {
    println!("{:?}", solution);
  }
}