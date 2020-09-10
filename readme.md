# Sudoku Solver in Rust

Blazing fast sudoku solver. Pre-implemented 4x4 and 9x9 boards. It's easy to
extend to even larger sudoku board.

## Use as a library

This crate can be used as a library

### 4 by 4 sudoku board example

``` rust
// Construct an empty 4x4 board
let board = Board4x4::default();

// It should generate 48 solutions for 4x4 sudoku board
for solution in board.solve() {
  println!("{}", solution);
}
```

### 9 by 9 sudoku board example

``` rust
// Construct an empty 4x4 board
let board = Board9x9::default();

// Since there are too many 9x9 solutions, we will only take the first 10
for solution in board.solve().take(10) {
  println!("{}", solution);
}
```

## Use as executables

To run executables, you can

```
cargo run --bin sudoku-gen -- --help
cargo run --bin sudoku-solve -- --help
```

You can install this package by doing

```
cargo install --path .
```

Then you will get two executables, `sudoku-gen` and `sudoku-solve`.

### Generating sudoku solution

Generate 50 9x9 sudoku solutions.

> Since there are way too many 9x9 sudoku solutions, you'd better add a
> `--num-solutions` constraint

```
sudoku-gen --board-size 9 --num-solutions 50
```

Generate all 4x4 sudoku solutions

```
sudoku-gen --board-size 4
```

Generate all 4x4 sudoku solutions and output to a json file

```
sudoku-gen --board-size 4 --output board4x4.json
```

The json file will look like this:

``` json
[
  [[3,1,2,4],[2,4,1,3],[4,2,3,1],[1,3,4,2]],
  [[3,1,2,4],[4,2,1,3],[2,4,3,1],[1,3,4,2]],
  // ...
]
```

### Generating sudoku question (with solution)

Generate 50 9x9 sudoku questions + solutions

```
sudoku-gen --board-size 9 --num-solutions 50 --generate-questions
```

Generate all 4x4 sudoku solutions with questions. Each solution generates 5
questions and 10 - 13 unknowns will be put inside the board.

```
sudoku-gen --board-size 4 \
           --generate-questions \
           --num-questions-per-solution 5 \
           --random-num-unknowns \
           --min-num-unknowns 10 \
           --max-num-unknowns 13 \
           --output board4x4questions.json
```

The output json file will look like this:

``` json
[
  {
    "q":[[1,0,0,0],[0,0,0,0],[4,0,0,3],[0,1,0,0]],
    "a":[[1,3,2,4],[2,4,3,1],[4,2,1,3],[3,1,4,2]]
  },
  {
    "q":[[0,0,0,0],[0,3,0,1],[0,2,0,0],[3,0,0,0]],
    "a":[[1,4,3,2],[2,3,4,1],[4,2,1,3],[3,1,2,4]]
  },
  // ...
]
```

### Solve existing sudoku problem

Directly pass in the data from command line

- 4x4:

```
sudoku-solve --board4x4 1 0 0 0 0 0 0 0 4 0 0 3 0 1 0 0
```

- 9x9:

Similar but you need to use `--board9x9` with 81 numbers supplied after it

You can also use input files:

```
sudoku-solve --input tests/boards/b9_1.json
```

with the input file looking like this:

``` json
[
  [5, 3, 0, 0, 7, 0, 0, 0, 0],
  [6, 0, 0, 1, 9, 5, 0, 0, 0],
  [0, 9, 8, 0, 0, 0, 0, 6, 0],
  [8, 0, 0, 0, 6, 0, 0, 0, 3],
  [4, 0, 0, 8, 0, 3, 0, 0, 1],
  [7, 0, 0, 0, 2, 0, 0, 0, 6],
  [0, 6, 0, 0, 0, 0, 2, 8, 0],
  [0, 0, 0, 4, 1, 9, 0, 0, 5],
  [0, 0, 0, 0, 8, 0, 0, 7, 9]
]
```