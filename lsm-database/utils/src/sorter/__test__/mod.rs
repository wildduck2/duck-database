#[cfg(test)]
mod sorter_test {
  use crate::sorter::Sorter;

  // --------------------
  // Selection sort tests
  // --------------------

  #[test]
  fn selection_sort_basic() {
    let vec = vec![7, 3, 5, 2];

    let result = Sorter::<u32>::selection_sort(vec.clone());
    assert_eq!(result, vec![2, 3, 5, 7]);
    assert_eq!(vec, vec![7, 3, 5, 2]);
  }

  #[test]
  fn selection_sort_with_duplicates() {
    let vec = vec![4, 1, 2, 1, 3];

    let result = Sorter::<u32>::selection_sort(vec);
    assert_eq!(result, vec![1, 1, 2, 3, 4]);
  }

  #[test]
  fn selection_sort_empty() {
    let vec: Vec<u32> = vec![];

    let result = Sorter::<u32>::selection_sort(vec);
    assert!(result.is_empty());
  }

  // ----------------
  // Quick sort tests
  // ----------------

  #[test]
  fn quick_sort_empty() {
    let v: Vec<i32> = vec![];
    let sorted = Sorter::quick_sort(v);
    assert_eq!(sorted, vec![]);
  }

  #[test]
  fn quick_sort_single_element() {
    let v = vec![42];
    let sorted = Sorter::quick_sort(v);
    assert_eq!(sorted, vec![42]);
  }

  #[test]
  fn quick_sort_two_elements() {
    let v = vec![2, 1];
    let sorted = Sorter::quick_sort(v);
    assert_eq!(sorted, vec![1, 2]);
  }

  // ----------------
  // Merge sort tests
  // ----------------

  #[test]
  fn merge_sort_empty() {
    let v: Vec<i32> = vec![];
    let sorted = Sorter::merge_sort(v);
    assert_eq!(sorted, vec![]);
  }

  #[test]
  fn merge_sort_single_element() {
    let v = vec![42];
    let sorted = Sorter::merge_sort(v);
    assert_eq!(sorted, vec![42]);
  }

  #[test]
  fn merge_sort_two_elements() {
    let v = vec![2, 1];
    let sorted = Sorter::merge_sort(v);
    assert_eq!(sorted, vec![1, 2]);
  }

  #[test]
  fn merge_sort_three_elements() {
    let v = vec![3, 1, 2];
    let sorted = Sorter::merge_sort(v);
    assert_eq!(sorted, vec![1, 2, 3]);
  }

  #[test]
  fn merge_sort_already_sorted() {
    let v = vec![1, 2, 3, 4, 5];
    let sorted = Sorter::merge_sort(v.clone());
    assert_eq!(sorted, v);
  }

  #[test]
  fn merge_sort_reverse_sorted() {
    let v = vec![5, 4, 3, 2, 1];
    let sorted = Sorter::merge_sort(v);
    assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
  }

  #[test]
  fn merge_sort_with_duplicates() {
    let v = vec![3, 1, 2, 3, 3, 0, 1];
    let sorted = Sorter::merge_sort(v);
    assert_eq!(sorted, vec![0, 1, 1, 2, 3, 3, 3]);
  }

  #[test]
  fn merge_sort_all_equal() {
    let v = vec![7, 7, 7, 7, 7];
    let sorted = Sorter::merge_sort(v);
    assert_eq!(sorted, vec![7, 7, 7, 7, 7]);
  }

  #[test]
  fn merge_sort_negative_numbers() {
    let v = vec![-5, -10, 0, 5, -1];
    let sorted = Sorter::merge_sort(v);
    assert_eq!(sorted, vec![-10, -5, -1, 0, 5]);
  }

  #[test]
  fn merge_sort_mixed_pattern() {
    let v = vec![10, 3, 7, 3, 2, 9, 1, 8];
    let sorted = Sorter::merge_sort(v);
    assert_eq!(sorted, vec![1, 2, 3, 3, 7, 8, 9, 10]);
  }

  #[test]
  fn merge_sort_large_input() {
    let mut v: Vec<i32> = (0..1000).rev().collect();
    let sorted = Sorter::merge_sort(v.clone());

    v.sort();
    assert_eq!(sorted, v);
  }

  #[test]
  fn merge_sort_strings() {
    let v = vec![
      "banana".to_string(),
      "apple".to_string(),
      "pear".to_string(),
      "apple".to_string(),
    ];

    let sorted = Sorter::merge_sort(v);

    assert_eq!(
      sorted,
      vec![
        "apple".to_string(),
        "apple".to_string(),
        "banana".to_string(),
        "pear".to_string(),
      ]
    );
  }

  // ----------------
  // Stability test
  // ----------------

  #[derive(Debug, PartialEq, Eq, Clone, Copy)]
  struct Item {
    key: i32,
    id: i32,
  }

  impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      self.key.cmp(&other.key)
    }
  }

  impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      Some(self.cmp(other))
    }
  }

  #[test]
  fn merge_sort_is_stable() {
    let v = vec![
      Item { key: 1, id: 1 },
      Item { key: 2, id: 1 },
      Item { key: 1, id: 2 },
      Item { key: 2, id: 2 },
    ];

    let sorted = Sorter::merge_sort(v);

    let ids: Vec<i32> = sorted
      .into_iter()
      .filter(|x| x.key == 1)
      .map(|x| x.id)
      .collect();

    assert_eq!(ids, vec![1, 2]);
  }

  #[test]
  fn merge_sort_preserves_length() {
    let v = vec![4, 2, 2, 9, 1, 4, 7];
    let sorted = Sorter::merge_sort(v.clone());
    assert_eq!(sorted.len(), v.len());
  }
}
