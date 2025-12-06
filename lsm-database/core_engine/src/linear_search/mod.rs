pub fn linear_search(data: Vec<u8>, x: u8) -> u8 {
  if data.is_empty() {
    return 0;
  }

  for i in 0..data.len() {
    if data[i] == x {
      return i as u8;
    }
  }
  0
}
