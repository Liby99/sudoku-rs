use sudoku_rs::{*, utils::Output};

use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "sudoku-solve")]
struct Options {

  /// The board content of a 4x4 sudoku problem. Column major. Need to supply
  /// 16 digits from 0 to 4
  #[structopt(long, multiple = true, number_of_values = 16)]
  board4x4: Option<Vec<u8>>,

  /// The board content of a 9x9 sudoku problem. Row major. Need to supply 81
  /// digits from 0 to 9
  #[structopt(long, multiple = true, number_of_values = 81)]
  board9x9: Option<Vec<u8>>,

  /// Use randomness in the solution generation process
  #[structopt(long)]
  no_random: bool,

  /// The random seed being used in the solution generation process
  #[structopt(long, name = "SEED")]
  seed: Option<u64>,

  /// The amount of solutions we want. If not specified, will fetch
  /// all possible solutions
  #[structopt(long, name = "#SOLUTIONS")]
  num_solutions: Option<usize>,

  /// Input json file name. If not specified, the input will come from command line
  #[structopt(short = "i", long, name = "FILE")]
  input: Option<String>,

  /// Output json file name. If not specified, the output will be printed on screen
  #[structopt(short = "o", long, name = "FILE")]
  output: Option<String>,
}

fn board_vec_from_file(input_file: &str) -> Result<Vec<u8>, String> {
  let mut file = File::open(input_file).map_err(|_| "Cannot open input file")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents).map_err(|_| "Cannot read from input file")?;
  let json : Vec<Vec<u8>> = serde_json::from_str(&contents).map_err(|_| "Cannot parse input file")?;
  let board_vec = json.into_iter().flatten().collect::<Vec<_>>();
  Ok(board_vec)
}

fn output_solution<B: Board>(output: &mut Output, i: usize, solution: B) -> Result<(), String> {
  // Output the separator
  if i > 0 {
    output.output_separator()?;
  }

  // If not, directly output the solution
  output.output_board(&solution)
}

fn execute_board<B: Board>(board: B, options: &Options) -> Result<(), String> {
  let mut output = Output::new(&options.output)?;

  // Initialize output
  output.output_init()?;

  let solution_iter = if options.no_random {
    board.solve_with_ctx(SolvingContext::deterministic())
  } else {
    match options.seed {
      Some(seed) => board.solve_with_ctx(SolvingContext::random_with_seed(seed)),
      _ => board.solve()
    }
  };

  // Generate solutions
  match options.num_solutions {
    Some(amount) => {
      for (i, solution) in solution_iter.take(amount).enumerate() {
        output_solution(&mut output, i, solution)?;
      }
    },
    _ => {
      for (i, solution) in solution_iter.enumerate() {
        output_solution(&mut output, i, solution)?;
      }
    }
  }

  // Finish output
  output.output_finish()
}

fn main() -> Result<(), String> {
  let options = Options::from_args();
  if let Some(input_file) = options.input.clone() {
    let board_vec = board_vec_from_file(&input_file)?;
    if board_vec.len() == 16 {
      let board = Board4x4::from_u8_vec(&board_vec);
      execute_board(board, &options)
    } else if board_vec.len() == 81 {
      let board = Board9x9::from_u8_vec(&board_vec);
      execute_board(board, &options)
    } else {
      Err("Sudoku file must contain 16 or 81 elements".to_string())
    }
  } else if let Some(b4) = options.board4x4.clone() {
    let board = Board4x4::from_u8_vec(&b4);
    execute_board(board, &options)
  } else if let Some(b9) = options.board9x9.clone() {
    let board = Board9x9::from_u8_vec(&b9);
    execute_board(board, &options)
  } else {
    Err("Must supply one of --input, --board4x4, or --board9x9".to_string())
  }
}