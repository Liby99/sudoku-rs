use rand::thread_rng;
use rand::seq::SliceRandom;

pub type Slot = (usize, usize);

pub trait BoardElement : Copy + Clone + Default + ToString {
  fn is_unknown(&self) -> bool;

  fn is_answer(&self) -> bool {
    !self.is_unknown()
  }
}

pub trait ElementSet : Default + Copy + Clone {
  type Element : BoardElement;

  fn intersect(&self, other: &Self) -> Self;

  fn complement(&self) -> Self;

  fn count(&self) -> usize;

  fn insert(&mut self, elem: &Self::Element);

  fn elements(&self) -> Vec<Self::Element>;
}

pub trait Board : Sized + Clone {
  type Element : BoardElement;

  type ElementSet : ElementSet<Element = Self::Element>;

  fn size() -> usize;

  fn block_size() -> usize;

  fn get(&self, slot: &Slot) -> &Self::Element;

  fn get_mut(&mut self, slot: &Slot) -> &mut Self::Element;

  fn set(&mut self, slot: &Slot, item: Self::Element) {
    *self.get_mut(slot) = item;
  }

  fn to_string(&self) -> String {
    let mut s = "[".to_string();
    for i in 0..Self::size() {
      for j in 0..Self::size() {
        s += &self.get(&(i, j)).to_string();
        if j < Self::size() - 1 {
          s += ","
        } else if i < Self::size() - 1 {
          s += ";\n "
        } else {
          s += ";\n "
        }
      }
    }
    s
  }

  fn slots() -> SlotIterator {
    SlotIterator { size: Self::size(), i: 0, j: 0 }
  }

  fn empty_slots(&self) -> Vec<Slot> {
    Self::slots().filter(|slot| self.get(&slot).is_unknown()).collect()
  }

  fn has_empty_slot(&self) -> bool {
    for slot in Self::slots() {
      if self.get(&slot).is_unknown() {
        return true;
      }
    }
    false
  }

  fn row_slots(row: usize) -> RowSlotIterator {
    RowSlotIterator { size: Self::size(), row, j: 0 }
  }

  fn column_slots(column: usize) -> ColumnSlotIterator {
    ColumnSlotIterator { size: Self::size(), i: 0, column }
  }

  fn block_slots(slot: &Slot) -> BlockSlotIterator {
    let x = slot.0 - slot.0 % Self::block_size();
    let y = slot.1 - slot.1 % Self::block_size();
    BlockSlotIterator { block_size: Self::block_size(), begin: (x, y), i: 0, j: 0 }
  }

  fn diagonal_slots(slot: &Slot) -> DiagonalSlotIterator {
    if slot.0 == slot.1 {
      DiagonalSlotIterator::Major { size: Self::size(), i: 0 }
    } else if slot.0 == Self::size() - slot.1 - 1 {
      DiagonalSlotIterator::Minor { size: Self::size(), i: 0 }
    } else {
      DiagonalSlotIterator::Nothing
    }
  }

  fn row_elements(&self, row: usize) -> Self::ElementSet {
    let mut set = Self::ElementSet::default();
    for slot in Self::row_slots(row) {
      set.insert(self.get(&slot))
    }
    set
  }

  fn column_elements(&self, column: usize) -> Self::ElementSet {
    let mut set = Self::ElementSet::default();
    for slot in Self::column_slots(column) {
      set.insert(self.get(&slot))
    }
    set
  }

  fn block_elements(&self, slot: &Slot) -> Self::ElementSet {
    let mut set = Self::ElementSet::default();
    for slot in Self::block_slots(slot) {
      set.insert(self.get(&slot))
    }
    set
  }

  fn diagonal_elements(&self, slot: &Slot) -> Self::ElementSet {
    let mut set = Self::ElementSet::default();
    for slot in Self::diagonal_slots(slot) {
      set.insert(self.get(&slot))
    }
    set
  }

  fn possible_answers(&self, slot: &Slot) -> Self::ElementSet {
    let row_ans = self.row_elements(slot.0).complement();
    let col_ans = self.column_elements(slot.1).complement();
    let blk_ans = self.block_elements(slot).complement();
    let diag_ans = self.diagonal_elements(slot).complement();
    row_ans.intersect(&col_ans).intersect(&blk_ans).intersect(&diag_ans)
  }

  fn put_random_unknowns(&mut self, amount: usize) {
    let mut all_slots = Self::slots().collect::<Vec<_>>();
    all_slots.shuffle(&mut thread_rng());
    for slot in &all_slots[..amount] {
      self.set(slot, Self::Element::default())
    }
  }

  fn put_determined_answers(&mut self) -> FillResult {
    let mut modified = false;
    for slot in Self::slots() {
      if self.get(&slot).is_unknown() {
        let answers = self.possible_answers(&slot);
        match answers.count() {
          0 => return FillResult::Unsatisfied,
          1 => {
            for answer in answers.elements() {
              self.set(&slot, answer);
              modified = true;
            }
          }
          _ => {}
        }
      }
    }
    if modified { FillResult::Modified }
    else { FillResult::Unmodified }
  }

  fn solve(self, shuffle: bool) -> BoardSolutions<Self> {
    BoardSolutions { stack: vec![self], shuffle }
  }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FillResult {
  Modified,
  Unmodified,
  Unsatisfied,
}

pub struct BoardSolutions<B> where B : Board {
  stack: Vec<B>,
  shuffle: bool,
}

impl<B> Iterator for BoardSolutions<B> where B : Board {
  type Item = B;

  fn next(&mut self) -> Option<Self::Item> {
    while !self.stack.is_empty() {
      // Since stack is not empty there must be a stack top
      let mut board = self.stack.pop().unwrap();

      // Fill in determined answers
      let mut fill_res = FillResult::Modified;
      if fill_res == FillResult::Modified {
        fill_res = board.put_determined_answers();
      }

      // Check if unsatisfied
      if fill_res == FillResult::Unsatisfied {
        continue;
      }

      // If still satisfied, check empty slots
      if board.has_empty_slot() {

        let mut least_constrained : Option<(usize, Slot, B::ElementSet)> = None;

        // Get all the empty slots
        let mut empty_slots = board.empty_slots();
        if self.shuffle { empty_slots.shuffle(&mut thread_rng()) }

        // Iterate all slots to find least constraint one
        for slot in empty_slots {
          let pos_answers = board.possible_answers(&slot);
          if least_constrained.is_none() || pos_answers.count() < least_constrained.unwrap().0 {
            least_constrained = Some((pos_answers.count(), slot, pos_answers));
          }
        }

        // Put all modified boards onto the stack
        if let Some((_, slot, pos_answers)) = least_constrained {

          // Get all the possible answers
          let mut pos_answers = pos_answers.elements();
          if self.shuffle { pos_answers.shuffle(&mut thread_rng()) }

          // Add all mutated boards onto the stack
          for pos_answer in pos_answers {
            let mut new_board = board.clone();
            new_board.set(&slot, pos_answer);
            self.stack.push(new_board);
          }
        }

      } else {
        return Some(board)
      }
    }
    None
  }
}

pub struct SlotIterator {
  size: usize,
  i: usize,
  j: usize,
}

impl Iterator for SlotIterator {
  type Item = Slot;

  fn next(&mut self) -> Option<Self::Item> {
    if self.j < self.size {
      let result = Some((self.i, self.j));
      self.j += 1;
      result
    } else {
      self.j = 0;
      if self.i < self.size {
        let result = Some((self.i, self.j));
        self.i += 1;
        result
      } else {
        None
      }
    }
  }
}

pub struct RowSlotIterator {
  size: usize,
  row: usize,
  j: usize,
}

impl Iterator for RowSlotIterator {
  type Item = Slot;

  fn next(&mut self) -> Option<Self::Item> {
    if self.j < self.size {
      let result = Some((self.row, self.j));
      self.j += 1;
      result
    } else {
      None
    }
  }
}

pub struct ColumnSlotIterator {
  size: usize,
  i: usize,
  column: usize,
}

impl Iterator for ColumnSlotIterator {
  type Item = Slot;

  fn next(&mut self) -> Option<Self::Item> {
    if self.i < self.size {
      let result = Some((self.i, self.column));
      self.i += 1;
      result
    } else {
      None
    }
  }
}

pub struct BlockSlotIterator {
  block_size: usize,
  begin: Slot,
  i: usize,
  j: usize
}

impl Iterator for BlockSlotIterator {
  type Item = Slot;

  fn next(&mut self) -> Option<Self::Item> {
    if self.j < self.block_size {
      let result = Some((self.i + self.begin.0, self.j + self.begin.1));
      self.j += 1;
      result
    } else {
      self.j = 0;
      if self.i < self.block_size {
        let result = Some((self.i + self.begin.0, self.j + self.begin.1));
        self.i += 1;
        result
      } else {
        None
      }
    }
  }
}

pub enum DiagonalSlotIterator {
  Major { size: usize, i: usize },
  Minor { size: usize, i: usize },
  Nothing
}

impl Iterator for DiagonalSlotIterator {
  type Item = Slot;

  fn next(&mut self) -> Option<Self::Item> {
    match self {
      Self::Major { size, i } => {
        if i < size {
          let result = Some((i.clone(), i.clone()));
          *i += 1;
          result
        } else {
          None
        }
      },
      Self::Minor { size, i } => {
        if i < size {
          let result = Some((i.clone(), size.clone() - i.clone() - 1));
          *i += 1;
          result
        } else {
          None
        }
      },
      Self::Nothing => None
    }
  }
}