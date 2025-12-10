//! A `Rc<RefCell<_>>` backed LIFO stack implementation.
//!
//! The stack stores nodes in a doubly linked list so pushes and pops stay at
//! `O(1)` while still allowing bidirectional iteration for debugging and tests.
//!
//! # Example
//!
//! ```rust
//! use utils::stack::Stack;
//!
//! let mut stack = Stack::new();
//! stack.push(10);
//! stack.push(20);
//!
//! assert_eq!(stack.peek_value(), Some(20));
//! stack.pop();
//! assert_eq!(stack.peek_value(), Some(10));
//! assert_eq!(stack.size(), 1);
//! ```

mod __test__;

use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// Internal node storing the stack value and neighbor pointers.
pub struct Node<T>
where
  T: PartialEq,
{
  pub prev: Link<T>,
  pub value: T,
  pub next: Link<T>,
}

impl<T> Node<T>
where
  T: PartialEq,
{
  fn new(value: T) -> Self {
    Self {
      prev: None,
      next: None,
      value,
    }
  }

  fn wrap(self) -> Rc<RefCell<Node<T>>> {
    Rc::new(RefCell::new(self))
  }
}

impl<T> std::fmt::Debug for Node<T>
where
  T: PartialEq + std::fmt::Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Node").field("value", &self.value).finish()
  }
}

/// A last-in-first-out stack maintained as a doubly linked list.
///
/// The structure owns its nodes via `Rc<RefCell<Node<T>>>`, enabling interior
/// mutability so pushes/pops can stitch nodes without copying the entire list.
pub struct Stack<T>
where
  T: PartialEq,
{
  head: Link<T>,
  tail: Link<T>,
  len: usize,
}

impl<T> Default for Stack<T>
where
  T: PartialEq,
{
  fn default() -> Self {
    Self {
      head: None,
      tail: None,
      len: 0,
    }
  }
}

impl<T> Stack<T>
where
  T: PartialEq + Copy,
{
  /// Creates a new empty stack.
  pub fn new() -> Self {
    Self::default()
  }

  /// Returns number of stored values.
  pub fn size(&self) -> usize {
    self.len
  }

  /// Returns `true` if stack stores no items.
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  /// Returns the node at the top of the stack without removing it.
  pub fn peek(&self) -> Link<T> {
    self.tail.clone()
  }

  /// Returns the value at the top of the stack without removing it.
  pub fn peek_value(&self) -> Option<T> {
    self.tail.as_ref().map(|rc| rc.borrow().value)
  }

  /// Removes all items, leaving the stack empty.
  pub fn clear(&mut self) {
    self.head = None;
    self.tail = None;
    self.len = 0;
  }

  /// Pushes a value onto the top of the stack and returns the new node handle.
  pub fn push(&mut self, value: T) -> Link<T> {
    let new = Node::new(value).wrap();

    match self.tail.take() {
      Some(old_tail) => {
        old_tail.borrow_mut().next = Some(new.clone());
        new.borrow_mut().prev = Some(old_tail);
        self.tail = Some(new.clone());
        if self.head.is_none() {
          self.head = self.tail.clone();
        }
      },
      None => {
        // List was empty
        self.head = Some(new.clone());
        self.tail = Some(new.clone());
      },
    }

    self.len += 1;
    self.tail.clone()
  }

  /// Pops the top value off the stack, returning the removed node.
  pub fn pop(&mut self) -> Link<T> {
    let old_tail = self.tail.clone()?;

    let prev = old_tail.borrow_mut().prev.take();
    match prev {
      Some(ref new_tail) => {
        new_tail.borrow_mut().next = None;
        self.tail = Some(new_tail.clone());
      },
      None => {
        // List becomes empty
        self.head = None;
        self.tail = None;
      },
    }

    self.len = self.len.saturating_sub(1);
    Some(old_tail)
  }

  /// Consumes the stack, returning the stored values from bottom to top.
  pub fn into_vec(self) -> Vec<T> {
    let mut vec = Vec::new();
    let mut cursor = self.head.clone();

    while let Some(node_rc) = cursor {
      let node = node_rc.borrow();
      vec.push(node.value);
      cursor = node.next.clone();
    }

    vec
  }

  /// Returns an iterator that yields values from bottom to top.
  pub fn iter(&self) -> StackIter<T> {
    StackIter {
      current: self.head.clone(),
    }
  }
}

/// Iterator over a [`Stack`], walking from the bottom (head) to the top.
pub struct StackIter<T>
where
  T: PartialEq,
{
  current: Link<T>,
}

impl<T> Iterator for StackIter<T>
where
  T: Clone + PartialEq,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    let current = self.current.clone()?;
    let value;
    let next;

    {
      let node = current.borrow();
      value = node.value.clone();
      next = node.next.clone();
    }

    self.current = next;
    Some(value)
  }
}

impl<T> std::fmt::Debug for Stack<T>
where
  T: PartialEq + std::fmt::Debug + Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.head.is_none() {
      return f.write_str("LinkedList [ empty ]");
    }

    let mut cursor = self.head.clone();
    let mut visited: Vec<*const RefCell<Node<T>>> = Vec::new();

    f.write_str("LinkedList [\n")?;

    while let Some(rc_node) = cursor {
      let raw = Rc::as_ptr(&rc_node);

      if visited.contains(&raw) {
        writeln!(f, "  cycle detected...")?;
        break;
      }
      visited.push(raw);

      let node = rc_node.borrow();

      let prev = node.prev.as_ref().map(|p| p.borrow().value.clone());
      let next = node.next.as_ref().map(|n| n.borrow().value.clone());

      writeln!(
        f,
        "  Node {{ prev: {:?}, value: {:?}, next: {:?} }}",
        prev, node.value, next
      )?;

      cursor = node.next.clone();
    }

    f.write_str("]")?;
    Ok(())
  }
}
