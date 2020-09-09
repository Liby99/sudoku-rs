use sudoku_rs::*;

#[test]
fn test_4x4_slots() {
  let v : Vec<_> = Board4x4::slots().collect();
  assert_eq!(v.len(), 16);
  assert_eq!(v[0], (0, 0));
  assert_eq!(v[1], (0, 1));
  assert_eq!(v[2], (0, 2));
  assert_eq!(v[3], (0, 3));
  assert_eq!(v[4], (1, 0));
  assert_eq!(v[5], (1, 1));
  assert_eq!(v[6], (1, 2));
  assert_eq!(v[7], (1, 3));
  assert_eq!(v[8], (2, 0));
  assert_eq!(v[9], (2, 1));
  assert_eq!(v[10], (2, 2));
  assert_eq!(v[11], (2, 3));
  assert_eq!(v[12], (3, 0));
  assert_eq!(v[13], (3, 1));
  assert_eq!(v[14], (3, 2));
  assert_eq!(v[15], (3, 3));
}

#[test]
fn test_4x4_row_slots() {
  let v : Vec<_> = Board4x4::row_slots(1).collect();
  assert_eq!(v.len(), 4);
  assert_eq!(v[0], (1, 0));
  assert_eq!(v[1], (1, 1));
  assert_eq!(v[2], (1, 2));
  assert_eq!(v[3], (1, 3));
}

#[test]
fn test_4x4_block_slots_1() {
  let v : Vec<_> = Board4x4::block_slots(&(0, 0)).collect();
  assert_eq!(v.len(), 4);
  assert_eq!(v[0], (0, 0));
  assert_eq!(v[1], (0, 1));
  assert_eq!(v[2], (1, 0));
  assert_eq!(v[3], (1, 1));
}

#[test]
fn test_4x4_block_slots_2() {
  let v : Vec<_> = Board4x4::block_slots(&(2, 2)).collect();
  assert_eq!(v.len(), 4);
  assert_eq!(v[0], (2, 2));
  assert_eq!(v[1], (2, 3));
  assert_eq!(v[2], (3, 2));
  assert_eq!(v[3], (3, 3));
}

#[test]
fn test_4x4_diagonal_slots_1() {
  let v : Vec<_> = Board4x4::diagonal_slots(&(0, 0)).collect();
  assert_eq!(v.len(), 4);
  assert_eq!(v[0], (0, 0));
  assert_eq!(v[1], (1, 1));
  assert_eq!(v[2], (2, 2));
  assert_eq!(v[3], (3, 3));
}

#[test]
fn test_4x4_diagonal_slots_2() {
  let v : Vec<_> = Board4x4::diagonal_slots(&(0, 3)).collect();
  assert_eq!(v.len(), 4);
  assert_eq!(v[0], (0, 3));
  assert_eq!(v[1], (1, 2));
  assert_eq!(v[2], (2, 1));
  assert_eq!(v[3], (3, 0));
}
