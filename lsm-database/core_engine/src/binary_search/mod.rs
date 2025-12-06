pub fn binary_search(data: Vec<u8>, x: u8) -> u8 {
  if data.is_empty() {
    return 0;
  }

  let mid = data.len() / 2;
  let current = data[mid];
  if x == current {
    return current;
  } else if x < current {
    let new = data[..mid].to_vec();
    return binary_search(new, x);
  } else {
    let new = data[mid + 1..].to_vec();
    return binary_search(new, x);
  }
}
