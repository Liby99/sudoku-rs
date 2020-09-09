use crate::board::*;

#[derive(Copy, Clone)]
pub enum Board9x9Element { U0, U1, U2, U3, U4, U5, U6, U7, U8, U9 }

impl Default for Board9x9Element {
  fn default() -> Self {
    Self::U0
  }
}

impl BoardElement for Board9x9Element {
  fn is_unknown(&self) -> bool {
    match self {
      Self::U0 => true,
      _ => false
    }
  }
}

#[derive(Clone)]
pub struct Board9x9 {
  board: [Board9x9Element; 81],
}

impl Default for Board9x9 {
  fn default() -> Self {
    Self { board: [Board9x9Element::default(); 81] }
  }
}

impl Board for Board9x9 {
  type Element = Board9x9Element;

  fn size() -> usize { 9 }

  fn block_size() -> usize { 3 }

  fn get(&self, slot: &Slot) -> &Self::Element {
    let index = slot.0 * 9 + slot.1;
    &self.board[index]
  }

  fn get_mut(&mut self, slot: &Slot) -> &mut Self::Element {
    let index = slot.0 * 9 + slot.1;
    &mut self.board[index]
  }
}
