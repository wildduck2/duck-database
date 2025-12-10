#[cfg(test)]
mod linked_list_test {
  use crate::linked_list::*;

  type TestList = LinkedList<&'static str>;

  fn build_list(values: &[&'static str]) -> TestList {
    let mut list = TestList::new();
    for value in values {
      list.insert_end(*value);
    }
    list
  }

  fn collect_values(list: &TestList) -> Vec<&'static str> {
    let mut values = Vec::new();
    let mut cursor = list.head.clone();

    while let Some(node) = cursor {
      let (value, next) = {
        let borrowed = node.borrow();
        (borrowed.value, borrowed.tail.clone())
      };
      values.push(value);
      cursor = next;
    }

    values
  }

  #[test]
  fn test_creation() {
    let list = TestList::new();
    assert!(list.head.is_none(), "An empty list");
    assert_eq!(list.size(), 0);
  }

  #[test]
  fn test_insert_end_appends_in_order() {
    let mut list = TestList::new();
    list.insert_end("i am the root node");
    list.insert_end("i am the second node");
    list.insert_end("i am the third node");

    assert_eq!(
      collect_values(&list),
      vec![
        "i am the root node",
        "i am the second node",
        "i am the third node"
      ]
    );

    // Tail of the last node should be None to terminate the chain
    let head = list.head.clone().unwrap();
    let second = head.borrow().tail.clone().unwrap();
    let third = second.borrow().tail.clone().unwrap();
    assert!(third.borrow().tail.is_none());
    assert_eq!(list.size(), 3);
  }

  #[test]
  fn test_size_tracks_insertions() {
    let mut list = TestList::new();

    let size = list.size();
    assert_eq!(size, 0);

    list.insert_end("i am the root node");
    list.insert_end("i am the second node");
    list.insert_end("i am the third node");
    list.insert_start("i am the new head");

    assert_eq!(list.size(), 4);
  }

  #[test]
  fn test_find_existing_and_missing_values() {
    let mut list = TestList::new();

    assert!(list.head.is_none(), "An empty list");

    list.insert_end("i am the root node");
    list.insert_end("i am the second node");
    list.insert_end("i am the third node");

    let root = list.find("i am the root node").unwrap();
    assert_eq!(root.borrow().value, "i am the root node");

    let third = list.find("i am the third node").unwrap();
    assert_eq!(third.borrow().value, "i am the third node");

    assert!(list.find("i do not exist").is_none());
  }

  #[test]
  fn test_insert_start_places_element_at_front() {
    let mut list = TestList::new();
    list.insert_end("root");
    list.insert_end("second");

    let new_head = list.insert_start("new head").unwrap();
    assert_eq!(new_head.borrow().value, "new head");
    assert_eq!(collect_values(&list), vec!["new head", "root", "second"]);
  }

  #[test]
  fn test_insert_at_handles_middle_and_end_positions() {
    let mut list = build_list(&["root", "second", "third"]);

    list.insert_at("new head", 0);
    assert_eq!(
      collect_values(&list),
      vec!["new head", "root", "second", "third"]
    );

    list.insert_at("middle", 2);
    assert_eq!(
      collect_values(&list),
      vec!["new head", "root", "middle", "second", "third"]
    );

    list.insert_at("trailing", 42);
    assert_eq!(
      collect_values(&list),
      vec!["new head", "root", "middle", "second", "third", "trailing"]
    );
  }

  #[test]
  fn test_node_at_returns_expected_nodes() {
    let list = build_list(&["root", "second", "third"]);
    let head = list.node_at(0).unwrap();
    assert_eq!(head.borrow().value, "root");

    let second = list.node_at(1).unwrap();
    assert_eq!(second.borrow().value, "second");

    assert!(list.node_at(99).is_none());
  }

  #[test]
  fn test_update_at_changes_value() {
    let mut list = build_list(&["root", "second", "third"]);

    let updated = list.update_at("second updated", 1).unwrap();
    assert_eq!(updated.borrow().value, "second updated");

    assert_eq!(
      collect_values(&list),
      vec!["root", "second updated", "third"]
    );

    assert!(list.update_at("out of range", 25).is_none());
  }
}
