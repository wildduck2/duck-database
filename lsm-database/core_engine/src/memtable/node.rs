pub enum Color {
  Red,
  Black,
}

pub struct Node<K, V> {
  pub key: K,
  pub value: V,
  pub color: Color,
  pub left: *mut Node<K, V>,
  pub right: *mut Node<K, V>,
  pub parent: *mut Node<K, V>,
}

impl<K: Default, V: Default> Node<K, V> {
  pub fn new(key: K, value: V, color: Color) -> Self {
    Self {
      key,
      value,
      color,
      parent: std::ptr::null_mut(),
      right: std::ptr::null_mut(),
      left: std::ptr::null_mut(),
    }
  }

  pub fn sentinel() -> Self {
    Self {
      key: K::default(),
      value: V::default(),
      color: Color::Black,
      parent: std::ptr::null_mut(),
      right: std::ptr::null_mut(),
      left: std::ptr::null_mut(),
    }
  }
}
