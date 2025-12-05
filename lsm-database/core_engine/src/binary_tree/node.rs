#[derive(Debug)]
pub struct Node<K, V>
where
  K: Default + Ord,
  V: Default,
{
  pub key: K,
  pub value: V,
  pub parent: Option<Box<Node<K, V>>>,
  pub left: Option<Box<Node<K, V>>>,
  pub right: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V>
where
  K: Default + Ord,
  V: Default,
{
  pub fn new(key: K, value: V) -> Self {
    Self {
      key,
      value,
      parent: None,
      left: None,
      right: None,
    }
  }
}
