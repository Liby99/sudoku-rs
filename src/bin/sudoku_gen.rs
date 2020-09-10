use sudoku_rs::*;

use structopt::StructOpt;
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::fs::File;
use std::io::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "sudoku_gen")]
struct Options {
  /// Board size. Could either be 4 or 9 for now
  #[structopt(long, default_value = "9", name = "SIZE")]
  board_size: usize,

  /// The amount of solutions we want. If not specified, will fetch
  /// all possible solutions
  #[structopt(long, name = "#SOLUTIONS")]
  num_solutions: Option<usize>,

  /// Use randomness in the solution generation process
  #[structopt(long)]
  no_random: bool,

  /// The random seed being used in the solution generation process
  #[structopt(long)]
  seed: Option<u64>,

  /// If specified, will generate questions instead of full solutions
  #[structopt(long)]
  generate_questions: bool,

  /// Number of questions being generated from one solution
  #[structopt(long, default_value = "1", name = "#QUESTIONS")]
  num_questions_per_solution: usize,

  /// Deterministic number of unknowns being put into each question
  #[structopt(long, name = "#UNKNOWNS", default_value = "12")]
  num_unknowns: usize,

  /// Use random amount of unknowns (given by [min_num_unknowns, max_num_unknowns])
  #[structopt(long)]
  random_num_unknowns: bool,

  /// Minimum number of unknowns when applying `random_num_unknowns`
  #[structopt(long, name = "MIN_#UNKNOWNS", default_value = "8")]
  min_num_unknowns: usize,

  /// Maximum number of unknowns when applying `random_num_unknowns`
  #[structopt(long, name = "MAX_#UNKNOWNS", default_value = "12")]
  max_num_unknowns: usize,

  /// Output json file name. If not specified, the output will be printed on screen
  #[structopt(short = "o", long)]
  output: Option<String>,
}

enum Output {
  File(File),
  Stdout,
}

impl Output {
  fn new(options: &Options) -> Result<Self, String> {
    match &options.output {
      Some(filename) => {
        let file = File::create(filename).map_err(|_| "Cannot create file")?;
        Ok(Self::File(file))
      },
      None => {
        Ok(Self::Stdout)
      }
    }
  }

  fn is_file(&self) -> bool {
    match self {
      Self::File(_) => true,
      _ => false,
    }
  }

  fn write(&mut self, s: &str) -> Result<(), String> {
    match self {
      Self::File(file) => {
        file.write_all(s.as_bytes()).map_err(|_| "Cannot write to file".to_string())
      },
      _ => {
        println!("{}", s);
        Ok(())
      }
    }
  }

  fn output_init(&mut self) -> Result<(), String> {
    if self.is_file() {
      self.write("[")
    } else {
      Ok(())
    }
  }

  fn output_separator(&mut self) -> Result<(), String> {
    if self.is_file() {
      self.write(",")
    } else {
      Ok(())
    }
  }

  fn output_finish(&mut self) -> Result<(), String> {
    if self.is_file() {
      self.write("]")
    } else {
      Ok(())
    }
  }

  fn board_to_json_str<B: Board>(board: &B) -> String {
    let mut s = "[".to_string();
    for row in 0..B::size() {
      if row > 0 { s += ","; }
      s += "[";
      for col in 0..B::size() {
        if col > 0 { s += ","; }
        s += board.get(&(row, col)).to_string().as_str();
      }
      s += "]";
    }
    s += "]";
    s
  }

  fn output_board<B: Board>(&mut self, board: &B) -> Result<(), String> {
    if self.is_file() {
      self.write(&Self::board_to_json_str(board))
    } else {
      self.write(&board.to_string())
    }
  }

  fn output_board_with_solution<B: Board>(&mut self, board: &B, solution: &B) -> Result<(), String> {
    if self.is_file() {
      self.write("{\"q\":")?;
      self.write(&Self::board_to_json_str(board))?;
      self.write(",\"a\":")?;
      self.write(&Self::board_to_json_str(solution))?;
      self.write("}")
    } else {
      self.write("Question: ")?;
      self.write(&board.to_string())?;
      self.write("Solution: ")?;
      self.write(&solution.to_string())
    }
  }
}

fn execute_on_board<B: Board>(board: B, options: Options) -> Result<(), String> {
  let mut output = Output::new(&options)?;
  let mut rng = match options.seed {
    Some(seed) => StdRng::seed_from_u64(seed),
    None => StdRng::from_entropy(),
  };

  // Initialize output
  output.output_init()?;

  // First get the solution iterator with a solving context constructed from options
  let solution_iter = if options.no_random {
    board.solve_with_ctx(SolvingContext::deterministic())
  } else {
    match options.seed {
      Some(seed) => board.solve_with_ctx(SolvingContext::random_with_seed(seed)),
      _ => board.solve()
    }
  };

  // Generate solutions
  let solutions : Vec<_> = match options.num_solutions {
    Some(amount) => solution_iter.take(amount).collect(),
    _ => solution_iter.collect()
  };

  // Output each solution
  for (i, solution) in solutions.into_iter().enumerate() {
    // Output the separator
    if i > 0 {
      output.output_separator()?;
    }

    // Check if we need to generate questions
    if options.generate_questions {

      // Find out the number of questions to generate
      for _ in 0..options.num_questions_per_solution {

        // Find out the number
        let num_unknowns = if options.random_num_unknowns {
          rng.gen_range(options.min_num_unknowns, options.max_num_unknowns)
        } else {
          options.num_unknowns
        };

        // Generate
        let mut question = solution.clone();
        question.put_random_unknowns(num_unknowns);

        // Output the question & solution
        output.output_board_with_solution(&question, &solution)?;
      }
    } else {

      // If not, directly output the solution
      output.output_board(&solution)?;
    }
  }

  // Finish output
  output.output_finish()?;

  Ok(())
}

fn main() -> Result<(), String> {
  let options = Options::from_args();
  if options.board_size == 4 {
    execute_on_board(Board4x4::default(), options)
  } else if options.board_size == 9 {
    execute_on_board(Board9x9::default(), options)
  } else {
    Err(format!("Unsupported board size {}", options.board_size))
  }
}