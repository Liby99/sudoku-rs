use rand::{thread_rng, SeedableRng, rngs::StdRng};
use rand::seq::SliceRandom;

/// Slot is a 2-tuple containing coordinate of the slot
pub type Slot = (usize, usize);

/// A board element needs to specify an `is_unknown` function.
/// This will be used when implementing the `Board` trait
pub trait BoardElement : Copy + Clone + Default + std::fmt::Debug + ToString {

  /// Check if this board element is unknown
  fn is_unknown(&self) -> bool;

  /// Check if this board element is an answer, basically it is the inverse of `is_unknown`
  fn is_answer(&self) -> bool {
    !self.is_unknown()
  }
}

/// An ElementSet is a set of elements, usually implemented in bitset format.
pub trait ElementSet : Default + Copy + Clone + std::fmt::Display {
  type Element : BoardElement;

  /// Get the intersection between two element sets
  fn intersect(&self, other: &Self) -> Self;

  /// Get the complement of this set over the whole set of Element
  fn complement(&self) -> Self;

  /// Count the number of elements inside the set
  fn count(&self) -> usize;

  /// Insert a new element into the set
  fn insert(&mut self, elem: &Self::Element);

  /// Get all the elements from the set, in a `Vec` form
  fn elements(&self) -> Vec<Self::Element>;
}

/// The trait for a Sudoku Board
pub trait Board : Sized + Clone + std::fmt::Debug {

  /// You need to specify the element that is being stored inside the board. It
  /// has to be a `BoardElement`
  type Element : BoardElement;

  /// You need to specify the element set type. Its internal Element type must
  /// be the same as the `Element` type for this `Board`
  type ElementSet : ElementSet<Element = Self::Element>;

  /// You need to specify the overall size of the sudoku board.
  /// e.g. The size for 4x4 board is 4; the size for 9x9 board is 9
  fn size() -> usize;

  /// You need to specify the size for a block inside the board
  /// e.g. The block_size for 4x4 board is 2; the block_size for 9x9 board is 3
  fn block_size() -> usize;

  /// You need to implement getting element by slot
  fn get(&self, slot: &Slot) -> &Self::Element;

  /// You need to implement getting mutable element by slot
  fn get_mut(&mut self, slot: &Slot) -> &mut Self::Element;

  /// Setting an item at the given slot
  fn set(&mut self, slot: &Slot, item: Self::Element) {
    *self.get_mut(slot) = item;
  }

  /// Turn the whole board into a string for display
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
          s += "]"
        }
      }
    }
    s
  }

  /// Get all the slots
  ///
  /// ```
  /// for slot in Board4x4::slots() {
  ///   // ...
  /// }
  /// ```
  fn slots() -> SlotIterator {
    SlotIterator { size: Self::size(), i: 0, j: 0 }
  }

  /// Get all the unknown slots (returns a vector of `Slot`)
  fn unknown_slots(&self) -> Vec<Slot> {
    Self::slots().filter(|slot| self.get(&slot).is_unknown()).collect()
  }

  /// Check if the board contains at least one unknown slot
  /// (slot containing unknown element value)
  fn has_unknown_slot(&self) -> bool {
    for slot in Self::slots() {
      if self.get(&slot).is_unknown() {
        return true;
      }
    }
    false
  }

  /// Get all the slots in the given row
  fn row_slots(row: usize) -> RowSlotIterator {
    RowSlotIterator { size: Self::size(), row, j: 0 }
  }

  /// Get all the slots in the given column
  fn column_slots(column: usize) -> ColumnSlotIterator {
    ColumnSlotIterator { size: Self::size(), i: 0, column }
  }

  /// Get all the slots in the block that the given slot resides in
  fn block_slots(slot: &Slot) -> BlockSlotIterator {
    let x = slot.0 - slot.0 % Self::block_size();
    let y = slot.1 - slot.1 % Self::block_size();
    BlockSlotIterator { block_size: Self::block_size(), begin: (x, y), i: 0, j: 0 }
  }

  /// Get the diagonal slots on the diagonal that the given slot resides in.
  ///
  /// There are several cases:
  /// - Slot is on both diagonals
  /// - Slot is on only major diagonal
  /// - Slot is on only minor diagonal
  /// - Slot is on neither
  fn diagonal_slots(slot: &Slot) -> DiagonalSlotIterator {
    let on_major = slot.0 == slot.1;
    let on_minor = slot.0 == Self::size() - slot.1 - 1;
    if on_major && on_minor {
      DiagonalSlotIterator::Both { size: Self::size(), major: true, i: 0 }
    } else if on_major {
      DiagonalSlotIterator::Major { size: Self::size(), i: 0 }
    } else if on_minor {
      DiagonalSlotIterator::Minor { size: Self::size(), i: 0 }
    } else {
      DiagonalSlotIterator::Nothing
    }
  }

  /// Get the element set for a given row
  fn row_elements(&self, row: usize) -> Self::ElementSet {
    let mut set = Self::ElementSet::default();
    for slot in Self::row_slots(row) {
      set.insert(self.get(&slot))
    }
    set
  }

  /// Get the element set for a given column
  fn column_elements(&self, column: usize) -> Self::ElementSet {
    let mut set = Self::ElementSet::default();
    for slot in Self::column_slots(column) {
      set.insert(self.get(&slot))
    }
    set
  }

  /// Get the element set for a block that the slot resides in
  fn block_elements(&self, slot: &Slot) -> Self::ElementSet {
    let mut set = Self::ElementSet::default();
    for slot in Self::block_slots(slot) {
      set.insert(self.get(&slot))
    }
    set
  }

  /// Get the diagonal element set for the diagonal(s) that the slot resides in
  fn diagonal_elements(&self, slot: &Slot) -> Self::ElementSet {
    let mut set = Self::ElementSet::default();
    for slot in Self::diagonal_slots(slot) {
      set.insert(self.get(&slot))
    }
    set
  }

  /// Get all the possible answers
  ///
  /// A possible answer for a given slot is a number that is not appearing inside
  /// its row, its column, its block, and its diagonal(s).
  ///
  /// Algorithm: get the elements on row, column, block, and diagonal, pick their
  /// complement, and then construct an intersection.
  fn possible_answers(&self, slot: &Slot) -> Self::ElementSet {
    let row_ans = self.row_elements(slot.0).complement();
    let col_ans = self.column_elements(slot.1).complement();
    let blk_ans = self.block_elements(slot).complement();
    row_ans.intersect(&col_ans).intersect(&blk_ans)

    // Ignoring diagonal rule for now
    // let diag_ans = self.diagonal_elements(slot).complement();
    // row_ans.intersect(&col_ans).intersect(&blk_ans).intersect(&diag_ans)
  }

  /// Put (`amount`) unknowns at random locations inside the board
  fn put_random_unknowns(&mut self, amount: usize) {
    let mut all_slots = Self::slots().collect::<Vec<_>>();
    all_slots.shuffle(&mut thread_rng());
    for slot in &all_slots[..amount] {
      self.set(slot, Self::Element::default())
    }
  }

  /// Find the slots that only contain one possible answer, and fill that answer
  /// in.
  ///
  /// If at least one slot is modified during this process, the result is "Modified"
  /// If no slot is modified, the result is "Unmodified"
  /// If there's one slot that contain no possible answer, then the result is "Unsatisfied"
  fn put_determined_answers(&mut self) -> FillResult {
    let mut modified = false;
    for slot in Self::slots() {
      if self.get(&slot).is_unknown() {
        let answers = self.possible_answers(&slot);
        match answers.count() {
          0 => {
            return FillResult::Unsatisfied
          },
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

  /// Solve with a solving context
  fn solve_with_ctx(&self, ctx: SolvingContext) -> BoardSolutions<Self> {
    BoardSolutions { stack: vec![self.clone()], ctx }
  }

  /// Solve the board by returning a solutions iterator.
  ///
  /// ```
  /// for solution in board.solve(true) {
  ///   // ...
  /// }
  /// ```
  fn solve(&self) -> BoardSolutions<Self> {
    self.solve_with_ctx(SolvingContext::default())
  }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FillResult {
  Modified,
  Unmodified,
  Unsatisfied,
}

pub enum SolvingContext {
  Deterministic,
  Random(StdRng),
}

impl Default for SolvingContext {
  fn default() -> Self {
    Self::Random(StdRng::from_entropy())
  }
}

impl SolvingContext {
  pub fn deterministic() -> Self {
    Self::Deterministic
  }

  pub fn random() -> Self {
    Self::Random(StdRng::from_entropy())
  }

  pub fn random_with_seed(seed: u64) -> Self {
    Self::Random(StdRng::seed_from_u64(seed))
  }

  pub fn is_random(&self) -> bool {
    match self {
      Self::Deterministic => false,
      Self::Random(_) => true,
    }
  }

  pub fn rng(&mut self) -> Option<&mut StdRng> {
    match self {
      Self::Deterministic => None,
      Self::Random(rng) => Some(rng)
    }
  }
}

/// Block solutions iterator
pub struct BoardSolutions<B> where B : Board {
  stack: Vec<B>,
  ctx: SolvingContext,
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
      if board.has_unknown_slot() {

        // Cache the least constraint element
        let mut least_constrained : Option<(usize, Slot, B::ElementSet)> = None;

        // Get all the empty slots
        let mut empty_slots = board.unknown_slots();
        if let Some(rng) = self.ctx.rng() { empty_slots.shuffle(rng) }

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
          if let Some(rng) = self.ctx.rng() { pos_answers.shuffle(rng) }

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
    if self.i < self.size {
      if self.j < self.size {
        let result = Some((self.i, self.j));
        self.j += 1;
        result
      } else {
        self.j = 0;
        self.i += 1;
        if self.i < self.size {
          let result = Some((self.i, self.j));
          self.j += 1;
          result
        } else {
          None
        }
      }
    } else {
      None
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
    if self.i < self.block_size {
      if self.j < self.block_size {
        let result = Some((self.i + self.begin.0, self.j + self.begin.1));
        self.j += 1;
        result
      } else {
        self.j = 0;
        self.i += 1;
        if self.i < self.block_size {
          let result = Some((self.i + self.begin.0, self.j + self.begin.1));
          self.j += 1;
          result
        } else {
          None
        }
      }
    } else {
      None
    }
  }
}

pub enum DiagonalSlotIterator {
  Both { size: usize, major: bool, i: usize },
  Major { size: usize, i: usize },
  Minor { size: usize, i: usize },
  Nothing
}

impl Iterator for DiagonalSlotIterator {
  type Item = Slot;

  fn next(&mut self) -> Option<Self::Item> {
    match self {
      Self::Both { size, major, i } => {
        if *major {
          if i < size {
            let result = Some((*i, *i));
            *i += 1;
            result
          } else {
            *major = false;
            *i = 1;
            Some((*i, *size - 1))
          }
        } else {
          if *i == *size - 1 - *i {
            *i += 1;
          }
          if i < size {
            let result = Some((*i, *size - *i - 1));
            *i += 1;
            result
          } else {
            None
          }
        }
      }
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