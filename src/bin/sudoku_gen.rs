use structopt::StructOpt;
use sudoku_rs::*;
use rand::{Rng, SeedableRng, rngs::StdRng};

#[derive(StructOpt, Debug)]
#[structopt(name = "sudoku_gen")]
struct Options {
  /// Board size. Could either be 4 or 9 for now
  #[structopt(long, default_value = "9", name = "SIZE")]
  board_size: usize,

  /// The amount of solutions we want. If not specified, will fetch
  /// all possible solutions
  #[structopt(long, name = "#SOLUTIONS")]
  solution_amount: Option<usize>,

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
  File {  },
  Stdout,
}

impl Output {
  fn new(options: &Options) -> Self {
    match &options.output {
      Some(filename) => {
        Self::File { }
      },
      None => {
        Self::Stdout
      }
    }
  }

  fn output_init(&self) -> Result<(), String> {
    Ok(())
  }

  fn output_separator(&self) -> Result<(), String> {
    Ok(())
  }

  fn output_finish(&self) -> Result<(), String> {
    Ok(())
  }

  fn output_board<B: Board>(&self, board: &B) -> Result<(), String> {
    match self {
      Self::File {  } => {
        Ok(())
      }
      Self::Stdout => {
        println!("");
        Ok(())
      }
    }
  }

  fn output_board_with_solution<B: Board>(&self, board: &B, solution: &B) -> Result<(), String> {
    match self {
      Self::File {  } => {
        Ok(())
      }
      Self::Stdout => {
        println!("");
        Ok(())
      }
    }
  }
}

fn execute_on_board<B: Board>(board: B, options: Options) -> Result<(), String> {
  let output = Output::new(&options);
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
  let solutions : Vec<_> = match options.solution_amount {
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