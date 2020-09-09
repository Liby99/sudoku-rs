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