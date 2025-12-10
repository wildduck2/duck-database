#[cfg(test)]
mod queue_test {
  use crate::queue::Queue;

  #[test]
  fn new_queue_is_empty() {
    let queue: Queue<i32> = Queue::new();
    assert_eq!(queue.size(), 0);
    assert!(queue.is_empty());
    assert!(queue.peek().is_none());
  }

  #[test]
  fn enqueue_and_peek_front() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    assert_eq!(queue.size(), 2);
    assert_eq!(queue.peek(), Some(1));
  }

  #[test]
  fn dequeue_returns_fifo_order() {
    let mut queue = Queue::new();
    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);

    assert_eq!(queue.dequeue(), Some(10));
    assert_eq!(queue.dequeue(), Some(20));
    assert_eq!(queue.dequeue(), Some(30));
    assert!(queue.is_empty());
  }

  #[test]
  fn dequeue_on_empty_returns_none() {
    let mut queue: Queue<i32> = Queue::new();
    assert!(queue.dequeue().is_none());
  }

  #[test]
  fn iter_and_vec_snapshot_preserve_fifo() {
    let mut queue = Queue::new();
    for i in 1..=4 {
      queue.enqueue(i);
    }

    let iter_values: Vec<_> = queue.iter().collect();
    assert_eq!(iter_values, vec![1, 2, 3, 4]);

    let snapshot = queue.into_vec();
    assert_eq!(snapshot, vec![1, 2, 3, 4]);
  }
}
