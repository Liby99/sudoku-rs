# Sudoku Solver in Rust

Blazing fast sudoku solver. Pre-implemented 4x4 and 9x9 boards. It's easy to
extend to even larger sudoku board.

## 4 by 4 sudoku board example

``` rust
// Construct an empty 4x4 board
let board = Board4x4::default();

// It should generate 48 solutions for 4x4 sudoku board
for solution in board.solve() {
  println!("{}", solution);
}
```

## 9 by 9 sudoku board example

``` rust
// Construct an empty 4x4 board
let board = Board9x9::default();

// Since there are too many 9x9 solutions, we will only take the first 10
for solution in board.solve().take(10) {
  println!("{}", solution);
}
```