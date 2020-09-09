use crate::board::*;

#[derive(Copy, Clone)]
pub enum Board4x4Element { U0, U1, U2, U3, U4 }

impl Default for Board4x4Element {
  fn default() -> Self {
    Self::U0
  }
}

impl BoardElement for Board4x4Element {
  fn is_unknown(&self) -> bool {
    match self {
      Self::U0 => true,
      _ => false
    }
  }
}

#[derive(Clone)]
pub struct Board4x4 {
  board: [Board4x4Element; 16],
}

impl Default for Board4x4 {
  fn default() -> Self {
    Self { board: [Board4x4Element::default(); 16] }
  }
}

impl Board for Board4x4 {
  type Element = Board4x4Element;

  fn size() -> usize { 4 }

  fn block_size() -> usize { 2 }

  fn get(&self, slot: &Slot) -> &Self::Element {
    let index = slot.0 * 4 + slot.1;
    &self.board[index]
  }

  fn get_mut(&mut self, slot: &Slot) -> &mut Self::Element {
    let index = slot.0 * 4 + slot.1;
    &mut self.board[index]
  }
}