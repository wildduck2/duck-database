mod __test__;

use std::{cell::RefCell, fmt, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
pub struct Node<T>
where
  T: Clone + PartialEq + std::fmt::Debug,
{
  pub head: Link<T>,
  pub value: T,
  pub tail: Link<T>,
}

impl<T> Node<T>
where
  T: Clone + PartialEq + std::fmt::Debug,
{
  pub fn new(value: T) -> Self {
    Self {
      head: None,
      value,
      tail: None,
    }
  }

  pub fn wrap(self) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(self))
  }
}

#[derive(Clone)]
pub struct LinkedList<T>
where
  T: Clone + PartialEq + std::fmt::Debug,
{
  head: Link<T>,
  len: usize,
}

impl<T> Default for LinkedList<T>
where
  T: Clone + PartialEq + std::fmt::Debug,
{
  fn default() -> Self {
    Self { head: None, len: 0 }
  }
}

impl<T> LinkedList<T>
where
  T: Clone + PartialEq + std::fmt::Debug,
{
  pub fn new() -> Self {
    Self::default()
  }

  pub fn find(&self, value: T) -> Link<T> {
    let mut cursor = self.head.clone()?;

    loop {
      if cursor.borrow().value == value {
        return Some(cursor.clone());
      }

      cursor = cursor.clone().borrow().tail.clone()?;
    }
  }

  pub fn size(&self) -> usize {
    self.len
  }

  pub fn node_at(&self, pos: usize) -> Link<T> {
    let mut cursor = match &self.head {
      Some(h) => h.clone(),
      None => return None,
    };

    let mut index = 0;

    loop {
      if index == pos {
        return Some(cursor.clone());
      }

      match cursor.clone().borrow().tail.clone() {
        Some(next) => {
          cursor = next;
          index += 1;
        },
        None => return None, // out of range
      }
    }
  }

  pub fn insert_start(&mut self, value: T) -> Link<T> {
    let old = self.head.clone();

    let new = Node::new(value).wrap();
    new.borrow_mut().tail = old;
    self.head = Some(new);

    self.len += 1;
    self.head.clone()
  }

  pub fn insert_end(&mut self, value: T) -> Link<T> {
    let new = Node::new(value).wrap();
    self.len += 1;

    match &self.head {
      Some(head) => {
        let mut cursor = head.clone();

        while let Some(next) = cursor.clone().borrow().tail.clone() {
          cursor = next;
        }

        // Link new node
        new.borrow_mut().head = Some(cursor.clone());
        cursor.borrow_mut().tail = Some(new.clone());

        Some(new)
      },

      None => {
        // First element
        self.head = Some(new.clone());
        Some(new)
      },
    }
  }

  pub fn insert_at(&mut self, value: T, pos: usize) -> Link<T> {
    let new = Node::new(value).wrap();

    let mut cursor = self.head.clone().unwrap();
    let mut current_pos = 0;

    while let Some(next) = cursor.clone().borrow().tail.clone() {
      if current_pos + 1 == pos {
        break;
      }
      cursor = next;
      current_pos += 1;
    }

    if pos == 0 {
      new.borrow_mut().tail = self.head.clone();
      self.head = Some(new.clone());
    } else {
      new.borrow_mut().tail = cursor.borrow().tail.clone();
      cursor.borrow_mut().tail = Some(new.clone());
    }

    if let Some(next) = new.borrow().tail.clone() {
      next.borrow_mut().head = Some(new.clone());
    }

    self.len += 1;
    Some(new)
  }

  pub fn update_at(&mut self, value: T, pos: usize) -> Link<T> {
    match Self::node_at(self, pos) {
      Some(x) => {
        x.borrow_mut().value = value;
        Some(x)
      },
      None => None,
    }
  }

  pub fn pop_start(&mut self) -> Link<T> {
    match &self.head {
      Some(head) => {
        let old = head.clone();

        old.borrow_mut().head = None;
        let node = old.borrow_mut().tail.clone();
        self.head = node.clone();
        if let Some(node) = node {
          node.borrow_mut().head = None;
        }

        Some(old)
      },
      None => None,
    }
  }

  pub fn pop_end(&mut self) -> Link<T> {
    match Self::node_at(self, self.len - 1) {
      Some(x) => {
        if let Some(node) = x.borrow_mut().head.clone() {
          node.borrow_mut().tail = None
        };

        Some(x)
      },
      None => {
        self.head = None;
        None
      },
    }
  }

  pub fn pop_at(&mut self, pos: usize) -> Link<T> {
    match Self::node_at(self, pos) {
      Some(x) => {
        println!("{:#?}", x);
        let head = x.borrow_mut().head.clone();
        let tail = x.borrow_mut().tail.clone();

        if let Some(h) = &head {
          h.borrow_mut().tail = tail.clone();
        }
        if let Some(t) = &tail {
          t.borrow_mut().head = head.clone();
        }

        if pos == 0 {
          self.head = tail.clone();
        }

        Some(x)
      },
      None => None,
    }
  }
}

impl<T> fmt::Debug for Node<T>
where
  T: Clone + PartialEq + fmt::Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Node").field("value", &self.value).finish()
  }
}

impl<T> fmt::Debug for LinkedList<T>
where
  T: Clone + PartialEq + fmt::Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let head_value = self.head.as_ref().map(|node| node.borrow().value.clone());
    let mut tail_value: Option<T> = None;
    let mut chain: Vec<String> = Vec::new();
    let mut node_views: Vec<NodeView<T>> = Vec::new();

    // store raw pointers to RefCell<Node<T>> for cycle detection
    let mut visited: Vec<*const RefCell<Node<T>>> = Vec::new();

    let mut curr = self.head.clone();

    while let Some(rc_node) = curr {
      let raw = Rc::as_ptr(&rc_node);

      // detect cycle
      if visited.contains(&raw) {
        chain.push("cycle detected".into());
        break;
      }

      visited.push(raw);

      let node = rc_node.borrow();
      tail_value = Some(node.value.clone());
      chain.push(format!("{:?}", node.value));

      let head_snapshot = node.head.as_ref().map(|head| head.borrow().value.clone());
      let tail_snapshot = node.tail.as_ref().map(|tail| tail.borrow().value.clone());

      node_views.push(NodeView {
        head: head_snapshot,
        value: node.value.clone(),
        tail: tail_snapshot,
      });

      curr = node.tail.clone();
    }

    f.debug_struct("LinkedList")
      .field("length", &self.len)
      .field("head", &head_value)
      .field("tail", &tail_value)
      .field("nodes", &node_views)
      .field("chain", &ChainDisplay { nodes: &chain })
      .finish()
  }
}

struct ChainDisplay<'a> {
  nodes: &'a [String],
}

impl<'a> fmt::Debug for ChainDisplay<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.nodes.is_empty() {
      write!(f, "Empty")
    } else {
      for (i, v) in self.nodes.iter().enumerate() {
        if i > 0 {
          write!(f, " -> ")?;
        }
        write!(f, "{}", v)?;
      }
      write!(f, " -> None")
    }
  }
}

struct NodeView<T>
where
  T: Clone + PartialEq + fmt::Debug,
{
  head: Option<T>,
  value: T,
  tail: Option<T>,
}

impl<T> fmt::Debug for NodeView<T>
where
  T: Clone + PartialEq + fmt::Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("NodeView")
      .field("head", &self.head)
      .field("value", &self.value)
      .field("tail", &self.tail)
      .finish()
  }
}
