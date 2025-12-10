#[cfg(test)]
mod linked_list_test {
  use crate::linked_list::*;

  type TestList = LinkedList<&'static str>;

  fn make_list() -> TestList {
    let mut list = TestList::new();
    list.insert_end("a");
    list.insert_end("b");
    list.insert_end("c");
    list.insert_end("d");
    list
  }

  // ---------------------------------------------------------
  // basic construction tests
  // ---------------------------------------------------------

  #[test]
  fn new_list_is_empty() {
    let list = TestList::new();
    assert_eq!(list.size(), 0);
    assert!(list.find("x").is_none());
    assert!(list.node_at(0).is_none());
  }

  #[test]
  fn insert_start_single() {
    let mut list = TestList::new();
    list.insert_start("a");
    assert_eq!(list.size(), 1);
    assert_eq!(list.node_at(0).unwrap().borrow()._value, "a");
  }

  #[test]
  fn insert_end_single() {
    let mut list = TestList::new();
    list.insert_end("a");
    assert_eq!(list.size(), 1);
    assert_eq!(list.node_at(0).unwrap().borrow()._value, "a");
  }

  // ---------------------------------------------------------
  // ordered insert tests
  // ---------------------------------------------------------

  #[test]
  fn insert_end_multiple() {
    let list = make_list();
    assert_eq!(list.size(), 4);
    assert_eq!(list.node_at(0).unwrap().borrow()._value, "a");
    assert_eq!(list.node_at(1).unwrap().borrow()._value, "b");
    assert_eq!(list.node_at(2).unwrap().borrow()._value, "c");
    assert_eq!(list.node_at(3).unwrap().borrow()._value, "d");
  }

  #[test]
  fn insert_at_middle() {
    let mut list = make_list();
    list.insert_at("x", 1); // a x b c d
    assert_eq!(list.node_at(1).unwrap().borrow()._value, "x");
    assert_eq!(list.node_at(2).unwrap().borrow()._value, "b");
    assert_eq!(list.node_at(3).unwrap().borrow()._value, "c");
  }

  #[test]
  fn insert_at_start() {
    let mut list = make_list();
    list.insert_at("z", 0);
    assert_eq!(list.node_at(0).unwrap().borrow()._value, "z");
    assert_eq!(list.node_at(1).unwrap().borrow()._value, "a");
  }

  #[test]
  fn insert_at_end() {
    let mut list = make_list();
    list.insert_at("x", 4);
    assert_eq!(list.node_at(4).unwrap().borrow()._value, "x");
  }

  // ---------------------------------------------------------
  // search tests
  // ---------------------------------------------------------

  #[test]
  fn find_existing_element() {
    let list = make_list();
    let node = list.find("c");
    assert!(node.is_some());
    assert_eq!(node.unwrap().borrow()._value, "c");
  }

  #[test]
  fn find_missing_element() {
    let list = make_list();
    assert!(list.find("z").is_none());
  }

  // ---------------------------------------------------------
  // update tests
  // ---------------------------------------------------------

  #[test]
  fn update_middle() {
    let mut list = make_list();
    list.update_at("x", 1);
    assert_eq!(list.node_at(1).unwrap().borrow()._value, "x");
  }

  #[test]
  fn update_start() {
    let mut list = make_list();
    list.update_at("x", 0);
    assert_eq!(list.node_at(0).unwrap().borrow()._value, "x");
  }

  #[test]
  fn update_out_of_bounds() {
    let mut list = make_list();
    assert!(list.update_at("x", 10).is_none());
  }

  // ---------------------------------------------------------
  // pop tests
  // ---------------------------------------------------------

  #[test]
  fn pop_start_basic() {
    let mut list = make_list();
    let removed = list.pop_start().unwrap();
    assert_eq!(removed.borrow()._value, "a");
    assert_eq!(list.size(), 3);
    assert_eq!(list.node_at(0).unwrap().borrow()._value, "b");
  }

  #[test]
  fn pop_end_basic() {
    let mut list = make_list();
    let removed = list.pop_end().unwrap();
    assert_eq!(removed.borrow()._value, "d");
    assert_eq!(list.size(), 3);
  }

  #[test]
  fn pop_at_middle() {
    let mut list = make_list();
    let removed = list.pop_at(1).unwrap(); // remove "b"
    assert_eq!(removed.borrow()._value, "b");
    assert_eq!(list.size(), 3);
    assert_eq!(list.node_at(1).unwrap().borrow()._value, "c");
  }

  #[test]
  fn pop_at_start() {
    let mut list = make_list();
    list.pop_at(0);
    assert_eq!(list.node_at(0).unwrap().borrow()._value, "b");
  }

  #[test]
  fn pop_at_end() {
    let mut list = make_list();
    list.pop_at(3);
    assert_eq!(list.size(), 3);
    assert_eq!(list.node_at(2).unwrap().borrow()._value, "c");
  }

  #[test]
  fn pop_from_single_element_list() {
    let mut list = TestList::new();
    list.insert_end("a");
    list.pop_end();
    assert_eq!(list.size(), 0);
    assert!(list.head.is_none());
  }

  #[test]
  fn pop_out_of_bounds() {
    let mut list = make_list();
    assert!(list.pop_at(10).is_none());
  }

  // ---------------------------------------------------------
  // iteration tests
  // ---------------------------------------------------------

  #[test]
  fn iterate_list_in_order() {
    let list = make_list();
    let collected: Vec<_> = list.iter().collect();
    assert_eq!(collected, vec!["a", "b", "c", "d"]);
  }

  #[test]
  fn iterate_empty_list() {
    let list = TestList::new();
    let collected: Vec<_> = list.iter().collect();
    assert!(collected.is_empty());
  }

  // ---------------------------------------------------------
  // structural integrity tests
  // ---------------------------------------------------------

  #[test]
  fn verify_bidirectional_links_after_insertions() {
    let list = make_list();
    let b = list.node_at(1).unwrap();
    let c = list.node_at(2).unwrap();

    assert_eq!(b.borrow().next.as_ref().unwrap().borrow()._value, "c");
    assert_eq!(c.borrow().prev.as_ref().unwrap().borrow()._value, "b");
  }

  #[test]
  fn verify_bidirectional_links_after_removal() {
    let mut list = make_list();
    list.pop_at(1); // remove "b"

    let a = list.node_at(0).unwrap();
    let c = list.node_at(1).unwrap();

    assert_eq!(a.borrow().next.as_ref().unwrap().borrow()._value, "c");
    assert_eq!(c.borrow().prev.as_ref().unwrap().borrow()._value, "a");
  }

  #[test]
  fn removing_all_elements_one_by_one() {
    let mut list = make_list();
    while list.size() > 0 {
      list.pop_end();
    }
    assert_eq!(list.size(), 0);
    assert!(list.head.is_none());
  }

  // ---------------------------------------------------------
  // stress tests
  // ---------------------------------------------------------

  #[test]
  fn large_list_insert_and_pop() {
    let mut list = LinkedList::new();

    for i in 0..1000 {
      list.insert_end(i.to_string());
    }

    assert_eq!(list.size(), 1000);

    for _ in 0..500 {
      list.pop_start();
    }

    assert_eq!(list.size(), 500);

    for _ in 0..250 {
      list.pop_end();
    }

    assert_eq!(list.size(), 250);
  }
}
