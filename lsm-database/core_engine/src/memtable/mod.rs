use crate::memtable::node::Node;

mod node;

// Decide how you represent null leaves. Usually a single shared sentinel node that is always black.
pub(crate) struct Tree<K, V> {
  pub root: *mut Node<K, V>,
  pub size: usize,
  pub sentinel: Box<Node<K, V>>,
}

impl<K: Default, V: Default> Tree<K, V> {
  pub(crate) fn new() -> Self {
    let mut sentinel = Box::new(Node::<K, V>::sentinel());
    let root = &mut *sentinel as *mut Node<K, V>;

    Self {
      root,
      sentinel,
      size: 0,
    }
  }

  pub(crate) fn rotation_left(&mut self, x: *mut Node<K, V>) {
    unsafe {
      let y = (*x).right;
      let s = self.sentinel.as_mut() as *mut Node<K, V>;

      (*x).right = (*y).left;

      if (*y).left != s {
        (*(*y).left).parent = x;
      }

      (*y).parent = (*x).parent;

      if (*x).parent == s {
        self.root = y;
      } else if (*(*x).parent).left == x {
        (*(*x).parent).left = y;
      } else {
        (*(*x).parent).right = y;
      }

      (*y).left = x;
      (*x).parent = y;
    }
  }

  pub(crate) fn rotation_right(&mut self, x: *mut Node<K, V>) {
    unsafe {
      let y = (*x).left;
      let s = self.sentinel.as_mut() as *mut Node<K, V>;

      (*x).left = (*y).right;

      if (*y).right != s {
        (*(*y).right).parent = x;
      }

      (*y).parent = (*x).parent;

      if (*x).parent == s {
        self.root = y;
      } else if (*(*x).parent).right == x {
        (*(*x).parent).right = y;
      } else {
        (*(*x).parent).left = y;
      }

      (*y).right = x;
      (*x).parent = y;
    }
  }
}

impl<K: Default, V: Default> Default for Tree<K, V> {
  fn default() -> Self {
    Self::new()
  }
}
