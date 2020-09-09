use crate::board::*;

#[derive(Copy, Clone)]
pub enum Board4x4Element { U0, U1, U2, U3, U4 }

impl From<u8> for Board4x4Element {
  fn from(n: u8) -> Self {
    match n {
      0 => Self::U0,
      1 => Self::U1,
      2 => Self::U2,
      3 => Self::U3,
      4 => Self::U4,
      _ => panic!("Invalid board 4x4 element {}", n)
    }
  }
}

impl ToString for Board4x4Element {
  fn to_string(&self) -> String {
    match self {
      Self::U0 => "0",
      Self::U1 => "1",
      Self::U2 => "2",
      Self::U3 => "3",
      Self::U4 => "4",
    }.to_string()
  }
}

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

#[derive(Copy, Clone, Default)]
pub struct Board4x4ElementSet(u8);

impl std::fmt::Display for Board4x4ElementSet {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_fmt(format_args!("{:0>8b}", self.0))
  }
}

impl ElementSet for Board4x4ElementSet {
  type Element = Board4x4Element;

  fn intersect(&self, other: &Self) -> Self {
    Self(self.0 & other.0)
  }

  fn complement(&self) -> Self {
    Self(!self.0)
  }

  fn count(&self) -> usize {
    let mut count = 0;
    for i in 0..4 {
      count += (self.0 >> i) & 1;
    }
    count as usize
  }

  fn insert(&mut self, elem: &Self::Element) {
    match elem {
      Self::Element::U1 => self.0 |= 1,
      Self::Element::U2 => self.0 |= 2,
      Self::Element::U3 => self.0 |= 4,
      Self::Element::U4 => self.0 |= 8,
      _ => {}
    }
  }

  fn elements(&self) -> Vec<Self::Element> {
    let mut result = vec![];
    if self.0 & 1 != 0 { result.push(Self::Element::U1) }
    if self.0 & 2 != 0 { result.push(Self::Element::U2) }
    if self.0 & 4 != 0 { result.push(Self::Element::U3) }
    if self.0 & 8 != 0 { result.push(Self::Element::U4) }
    result
  }
}

#[derive(Clone)]
pub struct Board4x4 {
  board: [Board4x4Element; 16],
}

impl Board4x4 {
  pub fn new(board: [Board4x4Element; 16]) -> Self {
    Self { board }
  }
}

impl Default for Board4x4 {
  fn default() -> Self {
    Self { board: [Board4x4Element::default(); 16] }
  }
}

impl std::fmt::Debug for Board4x4 {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str(&self.to_string())
  }
}

#[macro_export]
macro_rules! sudoku4x4 {
  ( $u11: literal , $u12: literal , $u13: literal , $u14: literal ;
    $u21: literal , $u22: literal , $u23: literal , $u24: literal ;
    $u31: literal , $u32: literal , $u33: literal , $u34: literal ;
    $u41: literal , $u42: literal , $u43: literal , $u44: literal $(;)? )
  => {
    {
      use Board4x4Element::*;
      Board4x4::new([
        Board4x4Element::from($u11), Board4x4Element::from($u12), Board4x4Element::from($u13), Board4x4Element::from($u14),
        Board4x4Element::from($u21), Board4x4Element::from($u22), Board4x4Element::from($u23), Board4x4Element::from($u24),
        Board4x4Element::from($u31), Board4x4Element::from($u32), Board4x4Element::from($u33), Board4x4Element::from($u34),
        Board4x4Element::from($u41), Board4x4Element::from($u42), Board4x4Element::from($u43), Board4x4Element::from($u44),
      ])
    }
  }
}

impl Board for Board4x4 {
  type Element = Board4x4Element;

  type ElementSet = Board4x4ElementSet;

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