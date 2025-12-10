//! A lightweight FIFO queue implemented with `Rc<RefCell<Node<T>>>` nodes.
//!
//! The queue mirrors the linked-list implementation powering the stack module,
//! which means `enqueue` and `dequeue` stay at `O(1)` while still allowing
//! iteration when desired.
//!
//! # Example
//!
//! ```rust
//! use utils::queue::Queue;
//!
//! let mut queue = Queue::new();
//! queue.enqueue(1);
//! queue.enqueue(2);
//!
//! assert_eq!(queue.peek(), Some(1));
//! assert_eq!(queue.dequeue(), Some(1));
//! assert_eq!(queue.dequeue(), Some(2));
//! assert!(queue.is_empty());
//! ```

mod __test__;

use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// A single node in the queue.
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

impl<T: std::fmt::Debug + PartialEq> std::fmt::Debug for Node<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Node").field("value", &self.value).finish()
  }
}

/// A FIFO queue implemented as a doubly linked list.
pub struct Queue<T>
where
  T: PartialEq,
{
  head: Link<T>,
  tail: Link<T>,
  len: usize,
}

impl<T> Default for Queue<T>
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

impl<T> Queue<T>
where
  T: PartialEq + Copy,
{
  /// Creates a new empty queue.
  pub fn new() -> Self {
    Self::default()
  }

  /// Returns `true` if queue is empty.
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  /// Returns number of items in queue.
  pub fn size(&self) -> usize {
    self.len
  }

  /// Returns the value at the front of the queue.
  pub fn peek(&self) -> Option<T> {
    self.head.as_ref().map(|rc| rc.borrow().value)
  }

  /// Pushes a value to the back of the queue.
  pub fn enqueue(&mut self, value: T) {
    let new = Node::new(value).wrap();

    match self.tail.take() {
      Some(old_tail) => {
        old_tail.borrow_mut().next = Some(new.clone());
        new.borrow_mut().prev = Some(old_tail);
        self.tail = Some(new);
      },
      None => {
        // Queue is empty
        self.head = Some(new.clone());
        self.tail = Some(new);
      },
    }

    self.len += 1;
  }

  /// Pops a value from the front of the queue.
  pub fn dequeue(&mut self) -> Option<T> {
    let old_head = self.head.clone()?;

    let value = old_head.borrow().value;

    let next = old_head.borrow_mut().next.take();

    match next {
      Some(new_head) => {
        new_head.borrow_mut().prev = None;
        self.head = Some(new_head);
      },
      None => {
        // Queue becomes empty
        self.head = None;
        self.tail = None;
      },
    }

    self.len -= 1;
    Some(value)
  }

  /// Converts the queue into a vec (FIFO order).
  pub fn into_vec(&self) -> Vec<T> {
    let mut result = Vec::new();
    let mut cursor = self.head.clone();

    while let Some(rc_node) = cursor {
      let node = rc_node.borrow();
      result.push(node.value);
      cursor = node.next.clone();
    }

    result
  }

  /// Returns an iterator over queue items (front → back).
  pub fn iter(&self) -> QueueIterator<T> {
    QueueIterator {
      current: self.head.clone(),
    }
  }
}

/// Iterator that walks the queue from front to back.
pub struct QueueIterator<T>
where
  T: PartialEq,
{
  current: Link<T>,
}

impl<T> Iterator for QueueIterator<T>
where
  T: Clone + PartialEq,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    let cur = self.current.clone()?;
    let value;
    let next;

    {
      let node = cur.borrow();
      value = node.value.clone();
      next = node.next.clone();
    }

    self.current = next;
    Some(value)
  }
}

impl<T> std::fmt::Debug for Queue<T>
where
  T: PartialEq + std::fmt::Debug + Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.head.is_none() {
      return f.write_str("Queue [ empty ]");
    }

    let mut cursor = self.head.clone();
    let mut visited: Vec<*const RefCell<Node<T>>> = Vec::new();

    f.write_str("Queue [\n")?;

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

    f.write_str("]")
  }
}
