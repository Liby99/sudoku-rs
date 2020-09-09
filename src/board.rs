pub type Slot = (usize, usize);

pub trait BoardElement : Copy + Clone {
  fn is_unknown(&self) -> bool;

  fn is_answer(&self) -> bool {
    !self.is_unknown()
  }
}

pub trait Board {
  type Element : BoardElement;

  fn size() -> usize;

  fn block_size() -> usize;

  fn get(&self, slot: &Slot) -> &Self::Element;

  fn get_mut(&mut self, slot: &Slot) -> &mut Self::Element;

  fn set(&mut self, slot: &Slot, item: Self::Element) {
    *self.get_mut(slot) = item;
  }

  fn slots() -> SlotIterator {
    SlotIterator { size: Self::size(), i: 0, j: 0 }
  }

  fn empty_slots(&self) -> Vec<Slot> {
    Self::slots().filter(|slot| self.get(&slot).is_unknown()).collect()
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

  fn has_empty_slot(&self) -> bool {
    for slot in Self::slots() {
      if self.get(&slot).is_unknown() {
        return true;
      }
    }
    false
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
