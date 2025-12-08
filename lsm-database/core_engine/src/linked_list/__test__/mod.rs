#[cfg(test)]
mod linked_list_test {
  use crate::linked_list::*;

  #[test]
  fn test_creation() {
    let list = LinkedList::<&str>::new();
    assert!(list.head.is_none(), "An empty list");
  }

  #[test]
  fn test_insert_end() {
    let mut list = LinkedList::<&str>::new();
    list.insert_end("i am the root node");

    let head = list.head.clone().unwrap();
    {
      let item = head.borrow();
      assert_eq!(item.value, "i am the root node");
      assert!(item.tail.is_none());
      assert!(item.head.is_none());
    }

    list.insert_end("i am the second node");
    list.insert_end("i am the third node");

    // Reborrow after the previous borrow ends
    let second = head.borrow().tail.clone().unwrap();
    let third = second.borrow().tail.clone().unwrap();

    assert_eq!(third.borrow().value, "i am the third node");
  }

  #[test]
  fn test_size() {
    let mut list = LinkedList::<&str>::new();

    let size = list.size();
    assert_eq!(size, 0);

    list.insert_end("i am the root node");
    list.insert_end("i am the second node");
    list.insert_end("i am the third node");

    let size = list.size();
    assert_eq!(size, 3);
  }

  #[test]
  fn test_find() {
    let mut list = LinkedList::<&str>::new();

    assert!(list.head.is_none(), "An empty list");

    list.insert_end("i am the root node");
    list.insert_end("i am the second node");
    list.insert_end("i am the third node");

    let item = list.find("i am the root node").unwrap();
    let item = item.borrow();
    assert_eq!(item.value, "i am the root node");

    // let item = list.find("i am the third node");
    // let item = list.find("i am the third node").unwrap();
    // let item = item.borrow();
    // assert_eq!(item.value, "i am the third node");
  }

  // #[test]
  // fn test_insert_start() {
  //   let mut list = LinkedList::<&str>::new();
  //
  //   let size = list.size();
  //   assert_eq!(size, 0);
  //
  //   list.insert_end("i am the root node");
  //   list.insert_end("i am the second node");
  //   list.insert_end("i am the third node");
  //   list.insert_at("i am the 4th node", 3);
  //
  //   let item = list.find("i am the 4th node").unwrap();
  //   let item = item.borrow();
  //   assert_eq!(item.value, "i am the 4th node");
  //   assert!(
  //     item.tail.is_none(),
  //     "This should be the last item in the list"
  //   );
  //
  //   let item = item.head.to_owned().unwrap();
  //   let item = item.borrow();
  //   assert_eq!(item.value, "i am the third node");
  //
  //   let size = list.size();
  //   assert_eq!(size, 4);
  // }
}
