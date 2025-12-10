//! A simple doubly linked list implemented with `Rc<RefCell<Node<T>>>`.
//!
//! This list keeps both `head` and `tail` pointers and supports:
//!
//! * inserting at the start, end, or at an index
//! * updating at the start, end, or at an index
//! * removing (popping) from the start, end, or at an index
//! * searching by value
//! * accessing a node by index
//! * iterating over all values in order
//!
//! # Example
//!
//! ```rust
//! use utils::linked_list::LinkedList;
//!
//! let mut list = LinkedList::new();
//! list.insert_end(1);
//! list.insert_end(2);
//! list.insert_end(3);
//!
//! assert_eq!(list.size(), 3);
//! assert_eq!(list.find(2).unwrap().borrow().value, 2);
//!
//! let values: Vec<_> = list.iter().collect();
//! assert_eq!(values, vec![1, 2, 3]);
//! ```

mod __test__;

use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

/// A single node in the linked list.
///
/// Each node stores:
///
/// * a previous pointer `prev`
/// * a value `value`
/// * a next pointer `next`
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
      value: value,
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

/// A doubly linked list with `head`, `tail`, and length.
///
/// The list owns its nodes through `Rc<RefCell<Node<T>>>`.
pub struct LinkedList<T>
where
  T: PartialEq,
{
  head: Link<T>,
  tail: Link<T>,
  len: usize,
}

impl<T> Default for LinkedList<T>
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

impl<T> LinkedList<T>
where
  T: PartialEq,
{
  /// Creates a new empty linked list.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use utils::linked_list::LinkedList;
  ///
  /// let list: LinkedList<i32> = LinkedList::new();
  /// assert_eq!(list.size(), 0);
  /// ```
  pub fn new() -> Self {
    Self::default()
  }

  /// Returns the number of elements in the list.
  pub fn size(&self) -> usize {
    self.len
  }

  /// Finds the first node whose value equals `value`.
  ///
  /// Returns `Some(node)` if found, or `None` if not present.
  pub fn find(&self, value: T) -> Link<T> {
    let mut cursor = self.head.clone();

    while let Some(node_rc) = cursor {
      let node = node_rc.borrow();
      if node.value == value {
        return Some(node_rc.clone());
      }
      cursor = node.next.clone();
    }

    None
  }

  /// Returns the node at the given index, if it exists.
  ///
  /// Indexes are zero based, from `0` to `len - 1`.
  ///
  /// Returns `None` if `index >= len`.
  pub fn node_at(&self, index: usize) -> Link<T> {
    if index >= self.len {
      return None;
    }

    let mut cursor = self.head.clone();
    let mut current = 0;

    while let Some(node_rc) = cursor.clone() {
      if current == index {
        return Some(node_rc);
      }
      cursor = node_rc.borrow().next.clone();
      current += 1;
    }

    None
  }

  /// Inserts a value at the start of the list.
  ///
  /// Returns a handle to the newly inserted node.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use utils::linked_list::LinkedList;
  ///
  /// let mut list = LinkedList::new();
  /// list.insert_start(1);
  /// list.insert_start(2);
  ///
  /// assert_eq!(list.size(), 2);
  /// let values: Vec<_> = list.iter().collect();
  /// assert_eq!(values, vec![2, 1]);
  /// ```
  pub fn insert_start(&mut self, value: T) -> Link<T> {
    let new = Node::new(value).wrap();

    match self.head.take() {
      Some(old_head) => {
        old_head.borrow_mut().prev = Some(new.clone());
        new.borrow_mut().next = Some(old_head);
        self.head = Some(new.clone());
      },
      None => {
        // List was empty, new node is both head and tail
        self.tail = Some(new.clone());
        self.head = Some(new.clone());
      },
    }

    self.len += 1;
    self.head.clone()
  }

  /// Inserts a value at the end of the list.
  ///
  /// Returns a handle to the newly inserted node.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use utils::linked_list::LinkedList;
  ///
  /// let mut list = LinkedList::new();
  /// list.insert_end(1);
  /// list.insert_end(2);
  ///
  /// let values: Vec<_> = list.iter().collect();
  /// assert_eq!(values, vec![1, 2]);
  /// ```
  pub fn insert_end(&mut self, value: T) -> Link<T> {
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

  /// Inserts a value at the given index.
  ///
  /// * If `index == 0`, inserts at the start.
  /// * If `index == len`, inserts at the end.
  /// * Otherwise inserts before the node currently at `index`.
  ///
  /// Returns a handle to the newly inserted node, or `None` if `index > len`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use utils::linked_list::LinkedList;
  ///
  /// let mut list = LinkedList::new();
  /// list.insert_end("a");
  /// list.insert_end("c");
  /// list.insert_at("b", 1);
  ///
  /// let values: Vec<_> = list.iter().collect();
  /// assert_eq!(values, vec!["a", "b", "c"]);
  /// ```
  pub fn insert_at(&mut self, value: T, index: usize) -> Link<T> {
    if index > self.len {
      return None;
    }

    if index == 0 {
      return self.insert_start(value);
    } else if index == self.len {
      return self.insert_end(value);
    }

    // index is between 1 and len - 1
    let current = self.node_at(index).unwrap();
    let prev = current.borrow().prev.clone().unwrap();

    let new = Node::new(value).wrap();

    {
      let mut prev_ref = prev.borrow_mut();
      prev_ref.next = Some(new.clone());
    }

    {
      let mut new_ref = new.borrow_mut();
      new_ref.prev = Some(prev.clone());
      new_ref.next = Some(current.clone());
    }

    {
      let mut cur_ref = current.borrow_mut();
      cur_ref.prev = Some(new.clone());
    }

    self.len += 1;
    Some(new)
  }

  /// Updates the value at the start of the list.
  ///
  /// Returns the updated node, or `None` if the list is empty.
  pub fn update_start(&mut self, value: T) -> Link<T> {
    match self.head.as_ref() {
      Some(head) => {
        head.borrow_mut().value = value;
        Some(head.clone())
      },
      None => None,
    }
  }

  /// Updates the value at the end of the list.
  ///
  /// Returns the updated node, or `None` if the list is empty.
  pub fn update_end(&mut self, value: T) -> Link<T> {
    match self.tail.as_ref() {
      Some(tail) => {
        tail.borrow_mut().value = value;
        Some(tail.clone())
      },
      None => None,
    }
  }

  /// Updates the value at the given index.
  ///
  /// Returns the updated node, or `None` if `index >= len`.
  pub fn update_at(&mut self, value: T, index: usize) -> Link<T> {
    if index >= self.len {
      return None;
    }

    if index == 0 {
      return self.update_start(value);
    } else if index == self.len - 1 {
      return self.update_end(value);
    }

    let node = self.node_at(index).unwrap();
    node.borrow_mut().value = value;
    Some(node)
  }

  /// Removes and returns the first node in the list.
  ///
  /// Returns `None` if the list is empty.
  pub fn pop_start(&mut self) -> Link<T> {
    let old_head = self.head.clone()?;

    let next = old_head.borrow_mut().next.take();
    match next {
      Some(ref new_head) => {
        new_head.borrow_mut().prev = None;
        self.head = Some(new_head.clone());
      },
      None => {
        // List becomes empty
        self.head = None;
        self.tail = None;
      },
    }

    self.len = self.len.saturating_sub(1);
    Some(old_head)
  }

  /// Removes and returns the last node in the list.
  ///
  /// Returns `None` if the list is empty.
  pub fn pop_end(&mut self) -> Link<T> {
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

  /// Removes and returns the node at the given index.
  ///
  /// * If `index == 0`, behaves like `pop_start`.
  /// * If `index == len - 1`, behaves like `pop_end`.
  /// * Otherwise unlinks the node in the middle.
  ///
  /// Returns `None` if `index >= len`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use utils::linked_list::LinkedList;
  ///
  /// let mut list = LinkedList::new();
  /// list.insert_end("a");
  /// list.insert_end("b");
  /// list.insert_end("c");
  ///
  /// let removed = list.pop_at(1).unwrap();
  /// assert_eq!(removed.borrow().value, "b");
  ///
  /// let values: Vec<_> = list.iter().collect();
  /// assert_eq!(values, vec!["a", "c"]);
  /// ```
  pub fn pop_at(&mut self, index: usize) -> Link<T> {
    if index >= self.len {
      return None;
    }

    if index == 0 {
      return self.pop_start();
    } else if index == self.len - 1 {
      return self.pop_end();
    }

    let current = self.node_at(index).unwrap();

    let prev = current.borrow().prev.clone();
    let next = current.borrow().next.clone();

    if let Some(ref p) = prev {
      p.borrow_mut().next = next.clone();
    }

    if let Some(ref n) = next {
      n.borrow_mut().prev = prev.clone();
    }

    self.len = self.len.saturating_sub(1);
    Some(current)
  }

  /// Returns an iterator over the values in the list, from head to tail.
  ///
  /// The iterator yields owned `T` values, so `T` must implement `Clone`.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use utils::linked_list::LinkedList;
  ///
  /// let mut list = LinkedList::new();
  /// list.insert_end(1);
  /// list.insert_end(2);
  /// list.insert_end(3);
  ///
  /// let values: Vec<_> = list.iter().collect();
  /// assert_eq!(values, vec![1, 2, 3]);
  /// ```
  pub fn iter(&self) -> LinkedListIter<T> {
    LinkedListIter {
      current: self.head.clone(),
    }
  }
}

/// Iterator over `LinkedList`, walking from head to tail.
pub struct LinkedListIter<T>
where
  T: PartialEq,
{
  current: Link<T>,
}

impl<T> Iterator for LinkedListIter<T>
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

impl<T> std::fmt::Debug for LinkedList<T>
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
