//! Sorting utilities that demonstrate algorithm implementations kept for
//! educational and reuse purposes across the workspace.
mod __test__;

/// A simple generic sorting helper that provides a selection sort
/// implementation for any type that implements `Ord` and `Copy`.
///
/// The type parameter `T` defaults to `u32`.  
/// This struct does not hold any data; it only serves as a namespace
/// for the sorting functions.
pub struct Sorter<T = u32>(std::marker::PhantomData<T>);

impl<T> Sorter<T>
where
  T: Ord + Clone,
{
  /// Sorts the input vector using the selection sort algorithm.
  ///
  /// Selection sort works by repeatedly finding the smallest value in the
  /// remaining unsorted portion of the vector, removing it, and placing it
  /// into a new result vector.  
  ///
  /// Steps:
  /// 1. Clone the input so the original data is never modified.
  /// 2. While the input still has elements:
  ///    - Scan through all elements to find the smallest one.
  ///    - Push that smallest value into the output.
  ///    - Remove the smallest element from the input vector.
  ///
  /// Time complexity:
  /// - Always O(n squared), because for each element it scans the entire
  ///   remaining list.
  ///
  /// Space complexity:
  /// - O(n), because it constructs a new result vector.
  ///
  /// Returns:
  /// - A new vector containing the sorted elements.
  ///
  /// Example:
  /// ```rust
  /// use utils::sorter::Sorter;
  ///
  /// let vec = vec![7, 3, 5, 2];
  ///
  /// let result = Sorter::<u32>::selection_sort(vec.clone());
  /// assert_eq!(result, vec![2, 3, 5, 7]);
  /// assert_eq!(vec, vec![7, 3, 5, 2]);
  /// ```
  ///
  pub fn selection_sort(mut data: Vec<T>) -> Vec<T> {
    let n = data.len();

    for i in 0..n {
      let mut min_idx = i;

      for j in (i + 1)..n {
        if data[j] < data[min_idx] {
          min_idx = j;
        }
      }

      data.swap(i, min_idx);
    }

    data
  }

  pub fn quick_sort(mut data: Vec<T>) -> Vec<T> {
    if data.len() < 2 {
      return data;
    }
    let pivot = data.pop().unwrap();
    let mut left = vec![];
    let mut right = vec![];

    for item in data {
      if item <= pivot {
        left.push(item);
      } else {
        right.push(item);
      }
    }

    let mut result = Sorter::quick_sort(left);
    result.push(pivot);
    result.extend(Sorter::quick_sort(right));

    result
  }

  pub fn merge_sort(data: Vec<T>) -> Vec<T> {
    if data.len() < 2 {
      return data;
    }

    let mid = data.len() / 2;
    let left = data[..mid].to_vec();
    let right = data[mid..].to_vec();

    let left = Sorter::merge_sort(left);
    let right = Sorter::merge_sort(right);

    Sorter::merge(left, right)
  }

  fn merge(mut left: Vec<T>, mut right: Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let i = 0;
    let j = 0;

    while i < left.len() && j < right.len() {
      if left[i] <= right[j] {
        result.push(left.remove(i));
      } else {
        result.push(right.remove(j));
      }
    }

    result.extend(left);
    result.extend(right);

    result
  }
}
