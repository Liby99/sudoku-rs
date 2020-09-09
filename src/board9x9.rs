use crate::board::*;

#[derive(Copy, Clone)]
pub enum Board9x9Element { U0, U1, U2, U3, U4, U5, U6, U7, U8, U9 }

impl ToString for Board9x9Element {
  fn to_string(&self) -> String {
    match self {
      Self::U0 => "0",
      Self::U1 => "1",
      Self::U2 => "2",
      Self::U3 => "3",
      Self::U4 => "4",
      Self::U5 => "5",
      Self::U6 => "6",
      Self::U7 => "7",
      Self::U8 => "8",
      Self::U9 => "9",
    }.to_string()
  }
}

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

#[derive(Copy, Clone, Default)]
pub struct Board9x9ElementSet(u16);

impl ElementSet for Board9x9ElementSet {
  type Element = Board9x9Element;

  fn intersect(&self, other: &Self) -> Self {
    Self(self.0 & other.0)
  }

  fn complement(&self) -> Self {
    Self(!self.0)
  }

  fn count(&self) -> usize {
    (self.0 & 1 + (self.0 >> 1) & 1 + (self.0 >> 2) & 1 + (self.0 >> 3) & 1 + (self.0 >> 4) & 1 + (self.0 >> 5) & 1 + (self.0 >> 6) & 1 + (self.0 >> 7) & 1 + (self.0 >> 8) & 1) as usize
  }

  fn insert(&mut self, elem: &Self::Element) {
    match elem {
      Self::Element::U1 => self.0 |= 1,
      Self::Element::U2 => self.0 |= 2,
      Self::Element::U3 => self.0 |= 4,
      Self::Element::U4 => self.0 |= 8,
      Self::Element::U5 => self.0 |= 16,
      Self::Element::U6 => self.0 |= 32,
      Self::Element::U7 => self.0 |= 64,
      Self::Element::U8 => self.0 |= 128,
      Self::Element::U9 => self.0 |= 256,
      _ => {}
    }
  }

  fn elements(&self) -> Vec<Self::Element> {
    let mut result = vec![];
    if self.0 & 1 != 0 { result.push(Self::Element::U1) }
    if self.0 & 2 != 0 { result.push(Self::Element::U2) }
    if self.0 & 4 != 0 { result.push(Self::Element::U3) }
    if self.0 & 8 != 0 { result.push(Self::Element::U4) }
    if self.0 & 16 != 0 { result.push(Self::Element::U5) }
    if self.0 & 32 != 0 { result.push(Self::Element::U6) }
    if self.0 & 64 != 0 { result.push(Self::Element::U7) }
    if self.0 & 128 != 0 { result.push(Self::Element::U8) }
    if self.0 & 256 != 0 { result.push(Self::Element::U9) }
    result
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

  type ElementSet = Board9x9ElementSet;

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
