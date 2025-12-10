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
  T: Ord + Copy,
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
  /// let result = Sorter::<u32>::selection_sort(&vec);
  /// assert_eq!(result, vec![2, 3, 5, 7]);
  /// assert_eq!(vec, vec![7, 3, 5, 2]);
  /// ```
  ///
  pub fn selection_sort(data: &Vec<T>) -> Vec<T> {
    let mut input = data.clone();
    let mut result = Vec::with_capacity(input.len());

    while !input.is_empty() {
      let mut min_idx = 0;

      for i in 0..input.len() {
        if input[i] < input[min_idx] {
          min_idx = i;
        }
      }

      result.push(input[min_idx]);
      input.remove(min_idx);
    }

    result
  }
}
