use crate::binary_tree::node::Node;

mod node;

pub struct BinaryTree<K, V>
where
  K: Default + Ord,
  V: Default,
{
  root: Option<Box<Node<K, V>>>,
  size: usize,
}

impl<K, V> BinaryTree<K, V>
where
  K: Default + Ord,
  V: Default,
{
  pub fn new() -> Self {
    let root = Some(Box::new(Node::new(K::default(), V::default())));

    Self { root, size: 0 }
  }

  pub fn insert(&mut self, key: K, value: V) {
    BinaryTree::insert_rec(&mut self.root, key, value);
    self.size += 1;
  }

  fn insert_rec(node: &mut Option<Box<Node<K, V>>>, key: K, value: V) {
    match node {
      Some(n) => {
        if key < n.key {
          BinaryTree::insert_rec(&mut n.left, key, value);
        } else if key > n.key {
          BinaryTree::insert_rec(&mut n.right, key, value);
        } else {
          n.value = value
        }
      }
      None => *node = Some(Box::new(Node::new(key, value))),
    }
  }

  pub fn delete(&mut self, key: K) {
    BinaryTree::delete_rec(&mut self.root, key);
    self.size -= 1;
  }

  fn delete_rec(node: &mut Option<Box<Node<K, V>>>, key: K) -> bool {
    let Some(n) = node else {
      return false;
    };

    if n.key < key {
      BinaryTree::delete_rec(&mut n.left, key);
    } else if n.key > key {
      BinaryTree::delete_rec(&mut n.right, key);
    }

    match (n.left.take(), n.right.take()) {
      (None, None) => *node = None,
      (None, Some(rn)) => *node = Some(rn),
      (Some(ln), None) => *node = Some(ln),
      (Some(ln), Some(rn)) => {
        let min = BinaryTree::extract_min(&mut Some(rn));
        *node = Some(min);
        BinaryTree::delete_rec(node, key);
      }
    }
    true
  }

  fn extract_min(node: &mut Option<Box<Node<K, V>>>) -> Box<Node<K, V>> {
    let mut n = node.take().unwrap();
    if n.left.is_none() {
      return n;
    }

    let min = BinaryTree::extract_min(&mut n.left);
    *node = Some(n);
    min
  }
}
