use crate::board::*;

#[derive(Copy, Clone)]
pub enum Board9x9Element { U0, U1, U2, U3, U4, U5, U6, U7, U8, U9 }

impl From<u8> for Board9x9Element {
  fn from(n: u8) -> Self {
    match n {
      0 => Self::U0,
      1 => Self::U1,
      2 => Self::U2,
      3 => Self::U3,
      4 => Self::U4,
      5 => Self::U5,
      6 => Self::U6,
      7 => Self::U7,
      8 => Self::U8,
      9 => Self::U9,
      _ => panic!("Invalid board 4x4 element {}", n)
    }
  }
}

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

impl std::fmt::Display for Board9x9ElementSet {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_fmt(format_args!("{:0>8b}", self.0))
  }
}

impl ElementSet for Board9x9ElementSet {
  type Element = Board9x9Element;

  fn intersect(&self, other: &Self) -> Self {
    Self(self.0 & other.0)
  }

  fn complement(&self) -> Self {
    Self(!self.0)
  }

  fn count(&self) -> usize {
    let mut count = 0;
    for i in 0..9 {
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

impl Board9x9 {
  pub fn new(board: [Board9x9Element; 81]) -> Self {
    Self { board }
  }
}

impl Default for Board9x9 {
  fn default() -> Self {
    Self { board: [Board9x9Element::default(); 81] }
  }
}

impl std::fmt::Debug for Board9x9 {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str(&self.to_string())
  }
}

#[macro_export]
macro_rules! sudoku9x9 {
  ( $u11: literal , $u12: literal , $u13: literal , $u14: literal , $u15: literal , $u16: literal , $u17: literal , $u18: literal , $u19: literal ;
    $u21: literal , $u22: literal , $u23: literal , $u24: literal , $u25: literal , $u26: literal , $u27: literal , $u28: literal , $u29: literal ;
    $u31: literal , $u32: literal , $u33: literal , $u34: literal , $u35: literal , $u36: literal , $u37: literal , $u38: literal , $u39: literal ;
    $u41: literal , $u42: literal , $u43: literal , $u44: literal , $u45: literal , $u46: literal , $u47: literal , $u48: literal , $u49: literal ;
    $u51: literal , $u52: literal , $u53: literal , $u54: literal , $u55: literal , $u56: literal , $u57: literal , $u58: literal , $u59: literal ;
    $u61: literal , $u62: literal , $u63: literal , $u64: literal , $u65: literal , $u66: literal , $u67: literal , $u68: literal , $u69: literal ;
    $u71: literal , $u72: literal , $u73: literal , $u74: literal , $u75: literal , $u76: literal , $u77: literal , $u78: literal , $u79: literal ;
    $u81: literal , $u82: literal , $u83: literal , $u84: literal , $u85: literal , $u86: literal , $u87: literal , $u88: literal , $u89: literal ;
    $u91: literal , $u92: literal , $u93: literal , $u94: literal , $u95: literal , $u96: literal , $u97: literal , $u98: literal , $u99: literal $(;)? )
  => {
    {
      type E = Board9x9Element;
      Board9x9::new([
        E::from($u11), E::from($u12), E::from($u13), E::from($u14), E::from($u15), E::from($u16), E::from($u17), E::from($u18), E::from($u19),
        E::from($u21), E::from($u22), E::from($u23), E::from($u24), E::from($u25), E::from($u26), E::from($u27), E::from($u28), E::from($u29),
        E::from($u31), E::from($u32), E::from($u33), E::from($u34), E::from($u35), E::from($u36), E::from($u37), E::from($u38), E::from($u39),
        E::from($u41), E::from($u42), E::from($u43), E::from($u44), E::from($u45), E::from($u46), E::from($u47), E::from($u48), E::from($u49),
        E::from($u51), E::from($u52), E::from($u53), E::from($u54), E::from($u55), E::from($u56), E::from($u57), E::from($u58), E::from($u59),
        E::from($u61), E::from($u62), E::from($u63), E::from($u64), E::from($u65), E::from($u66), E::from($u67), E::from($u68), E::from($u69),
        E::from($u71), E::from($u72), E::from($u73), E::from($u74), E::from($u75), E::from($u76), E::from($u77), E::from($u78), E::from($u79),
        E::from($u81), E::from($u82), E::from($u83), E::from($u84), E::from($u85), E::from($u86), E::from($u87), E::from($u88), E::from($u89),
        E::from($u91), E::from($u92), E::from($u93), E::from($u94), E::from($u95), E::from($u96), E::from($u97), E::from($u98), E::from($u99),
      ])
    }
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
