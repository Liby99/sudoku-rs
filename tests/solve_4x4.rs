use sudoku_rs::*;

#[test]
fn solve_4x4() {
  let board = Board4x4::default();
  let mut solutions = board.solve(true);
  println!("{}", solutions.next().unwrap().to_string());
  println!("{}", solutions.next().unwrap().to_string());
  println!("{}", solutions.next().unwrap().to_string());
}

#[test]
fn solve_pre_filled_4x4() {
  let board = sudoku4x4![
    1, 2, 3, 4;
    3, 4, 1, 2;
    4, 3, 2, 1;
    2, 1, 4, 3;
  ];
  let mut solutions = board.solve(true);
  assert!(solutions.next().is_some());
  assert!(solutions.next().is_none());
}

#[test]
fn solve_pre_filled_4x4_digged() {
  let mut board = sudoku4x4![
    1, 2, 3, 4;
    3, 4, 1, 2;
    4, 3, 2, 1;
    2, 1, 4, 3;
  ];
  board.put_random_unknowns(13);
  println!("Digged: {:?}", board);
  for solution in board.solve(true) {
    println!("{:?}", solution);
  }
}