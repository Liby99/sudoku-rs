use std::fs::File;
use std::io::prelude::*;

use crate::Board;

pub enum Output {
  File(File),
  Stdout,
}

impl Output {
  pub fn new(maybe_file: &Option<String>) -> Result<Self, String> {
    match maybe_file {
      Some(filename) => {
        let file = File::create(filename).map_err(|_| "Cannot create file")?;
        Ok(Self::File(file))
      },
      None => {
        Ok(Self::Stdout)
      }
    }
  }

  pub fn is_file(&self) -> bool {
    match self {
      Self::File(_) => true,
      _ => false,
    }
  }

  pub fn write(&mut self, s: &str) -> Result<(), String> {
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

  pub fn output_init(&mut self) -> Result<(), String> {
    if self.is_file() {
      self.write("[")
    } else {
      Ok(())
    }
  }

  pub fn output_separator(&mut self) -> Result<(), String> {
    if self.is_file() {
      self.write(",")
    } else {
      Ok(())
    }
  }

  pub fn output_finish(&mut self) -> Result<(), String> {
    if self.is_file() {
      self.write("]")
    } else {
      Ok(())
    }
  }

  pub fn board_to_json_str<B: Board>(board: &B) -> String {
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

  pub fn output_board<B: Board>(&mut self, board: &B) -> Result<(), String> {
    if self.is_file() {
      self.write(&Self::board_to_json_str(board))
    } else {
      self.write(&board.to_string())
    }
  }

  pub fn output_board_with_solution<B: Board>(&mut self, board: &B, solution: &B) -> Result<(), String> {
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