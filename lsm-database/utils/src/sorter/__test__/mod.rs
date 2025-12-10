#[cfg(test)]
mod sorter_test {

  use crate::sorter::*;

  #[test]
  fn test_selection_sort() {
    let vec = vec![7, 3, 5, 2];

    let result = Sorter::<u32>::selection_sort(&vec);
    assert_eq!(result, vec![2, 3, 5, 7]);
    assert_eq!(vec, vec![7, 3, 5, 2]);
  }

  #[test]
  fn test_selection_sort_with_duplicates() {
    let vec = vec![4, 1, 2, 1, 3];

    let result = Sorter::<u32>::selection_sort(&vec);
    assert_eq!(result, vec![1, 1, 2, 3, 4]);
  }

  #[test]
  fn test_selection_sort_with_empty_input() {
    let vec: Vec<u32> = vec![];

    let result = Sorter::<u32>::selection_sort(&vec);
    assert!(result.is_empty());
  }
}
