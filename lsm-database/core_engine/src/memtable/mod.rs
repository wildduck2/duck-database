use std::thread::current;

use crate::memtable::node::{Color, Node};

mod node;

// Decide how you represent null leaves. Usually a single shared sentinel node that is always black.
pub(crate) struct RBTree<K, V> {
  pub root: *mut Node<K, V>,
  pub size: usize,
  pub sentinel: *mut Node<K, V>,
}

impl<K, V> RBTree<K, V>
where
  K: Default + Ord,
  V: Default,
{
  pub(crate) fn new() -> Self {
    let mut s = Box::new(Node::<K, V>::sentinel());
    let s_ptr: *mut _ = &mut *s;

    unsafe {
      (*s_ptr).parent = s_ptr;
      (*s_ptr).left = s_ptr;
      (*s_ptr).right = s_ptr;
    }

    // Leak the sentinel so the pointer stays alive
    std::mem::forget(s);

    Self {
      root: s_ptr,
      sentinel: s_ptr,
      size: 0,
    }
  }

  fn insert(&mut self, key: K, value: V) {
    unsafe {
      let mut node = Box::new(Node::new(key, value, Color::Red));
      let node_ptr: *mut _ = node.as_mut();

      (*node_ptr).left = self.sentinel;
      (*node_ptr).right = self.sentinel;

      let s = self.sentinel;
      let mut parent = s;
      let mut current = self.root;

      while !self.is_sentinel(current) {
        parent = current;

        if (*node_ptr).key < (*current).key {
          current = (*current).left;
        } else if (*node_ptr).key > (*current).key {
          current = (*current).right;
        } else {
          (*current).value = std::mem::take(&mut (*node_ptr).value);
          drop(node);
          return;
        }
      }

      (*node_ptr).parent = parent;

      if self.is_sentinel(parent) {
        self.root = node_ptr;
      } else if (*node_ptr).key < (*parent).key {
        (*parent).left = node_ptr;
      } else {
        (*parent).right = node_ptr;
      }

      (*self.root).parent = s;
      drop(node);

      self.fix_insert(node_ptr);
    }
  }

  fn is_sentinel(&self, s: *mut Node<K, V>) -> bool {
    self.sentinel == s
  }

  fn fix_insert(&mut self, mut node: *mut Node<K, V>) {
    unsafe {}
  }

  fn rotation_left(&mut self, x: *mut Node<K, V>) {
    unsafe {
      let y = (*x).right;
      (*x).right = (*y).left;

      if !self.is_sentinel((*y).left) {
        (*(*y).left).parent = x;
      }

      (*y).parent = (*x).parent;

      if self.is_sentinel((*x).parent) {
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

  fn rotation_right(&mut self, x: *mut Node<K, V>) {
    unsafe {
      let y = (*x).left;
      (*x).left = (*y).right;

      if !self.is_sentinel((*y).right) {
        (*(*y).right).parent = x;
      }

      (*y).parent = (*x).parent;

      if self.is_sentinel((*x).parent) {
        self.root = y;
      } else if x == (*(*x).parent).right {
        (*(*x).parent).right = y;
      } else {
        (*(*x).parent).left = y;
      }

      (*y).right = x;
      (*x).parent = y;
    }
  }
}
