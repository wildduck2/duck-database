#[cfg(test)]
mod searcher_test {

  use crate::searcher::*;

  #[test]
  fn test_linear_search() {
    let vec = vec![7, 3, 5, 2];

    let result = Searcher::<u32>::linear_search(&vec, 3);
    assert_eq!(result, Some(3));
  }

  #[test]
  fn test_binary_search() {
    let vec = vec![2, 3, 5, 7, 11, 13];

    let result = Searcher::<u32>::binary_search(&vec, 3);
    assert_eq!(result, Some(3));
  }

  #[test]
  fn test_linear_search_missing_value() {
    let vec = vec![7, 3, 5, 2];

    let result = Searcher::<u32>::linear_search(&vec, 4);
    assert_eq!(result, None);
  }

  #[test]
  fn test_binary_search_left_branch() {
    let vec = vec![2, 3, 5, 7, 11, 13];

    let result = Searcher::<u32>::binary_search(&vec, 2);
    assert_eq!(result, Some(2));
  }

  #[test]
  fn test_binary_search_missing_left_branch() {
    let vec = vec![2, 3, 5, 7, 11, 13];

    let result = Searcher::<u32>::binary_search(&vec, 1);
    assert_eq!(result, None);
  }
}
