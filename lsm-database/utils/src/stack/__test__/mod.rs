#[cfg(test)]
mod stack_test {
  use crate::stack::Stack;

  #[test]
  fn new_stack_is_empty() {
    let stack: Stack<i32> = Stack::new();
    assert_eq!(stack.size(), 0);
    assert!(stack.is_empty());
    assert!(stack.peek_value().is_none());
    assert!(stack.peek().is_none());
  }

  #[test]
  fn push_and_peek_updates_top() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.peek_value(), Some(2));
    let node = stack.peek().unwrap();
    assert_eq!(node.borrow().value, 2);
  }

  #[test]
  fn pop_follows_lifo() {
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);

    let first = stack.pop().unwrap();
    assert_eq!(first.borrow().value, 30);
    assert_eq!(stack.size(), 2);

    let second = stack.pop().unwrap();
    assert_eq!(second.borrow().value, 20);
    assert_eq!(stack.peek_value(), Some(10));
  }

  #[test]
  fn pop_on_empty_returns_none() {
    let mut stack: Stack<i32> = Stack::new();
    assert!(stack.pop().is_none());
  }

  #[test]
  fn clear_resets_stack() {
    let mut stack = Stack::new();
    for i in 0..5 {
      stack.push(i);
    }

    stack.clear();
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);
    assert!(stack.peek_value().is_none());
  }

  #[test]
  fn iterator_and_vec_snapshot_match() {
    let mut stack = Stack::new();
    for i in 1..=4 {
      stack.push(i);
    }

    let iter_values: Vec<_> = stack.iter().collect();
    assert_eq!(iter_values, vec![1, 2, 3, 4]);

    let as_vec = stack.into_vec();
    assert_eq!(as_vec, vec![1, 2, 3, 4]);
  }
}
