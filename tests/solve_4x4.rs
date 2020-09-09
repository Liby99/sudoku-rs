use sudoku_rs::*;

#[test]
fn solve_4x4() {
  let board = Board4x4::default();
  let mut solutions = board.solve(true);
  println!("{}", solutions.next().unwrap().to_string());
  println!("{}", solutions.next().unwrap().to_string());
  println!("{}", solutions.next().unwrap().to_string());
}