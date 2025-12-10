//! Search utilities that expose reusable linear and binary search helpers.
//!
//! The module intentionally keeps the API surface small so it can be embedded
//! anywhere in the workspace that needs simple search behavior without pulling
//! in external crates.
mod __test__;

/// A generic search helper that provides linear and binary search functions.
///
/// The type parameter `T` defaults to `u32`.  
/// It must implement:
/// - `Ord` so values can be compared for ordering
/// - `Copy` so results can be returned by value
///
/// This struct does not store any data.  
/// It only carries the type information needed for the search functions.
pub struct Searcher<T = u32>(std::marker::PhantomData<T>);

impl<T> Searcher<T>
where
  T: Copy + Ord,
{
  /// Performs a simple linear search on the vector.
  ///
  /// This function checks each element from left to right until it finds
  /// the same value.  
  /// If the value appears multiple times, the last matching value is returned.
  ///
  /// Time complexity:
  /// - Best case: O(1)
  /// - Worst case: O(n)
  ///
  /// Returns:
  /// - Some(value) if the value exists in the vector
  /// - None if the value does not appear
  ///
  /// Example:
  /// ```rust
  /// use utils::searcher::Searcher;
  ///
  /// let data = vec![1, 2, 3, 4, 5];
  ///
  /// assert_eq!(Searcher::<u32>::linear_search(&data, 1), Some(1));
  /// assert_eq!(Searcher::<u32>::linear_search(&data, 6), None);
  /// ```
  ///
  pub fn linear_search(data: &Vec<T>, value: T) -> Option<T> {
    let mut result = None;

    for item in data {
      if *item == value {
        result = Some(*item);
      }
    }

    result
  }

  /// Performs a binary search on the vector.
  ///
  /// Important:
  /// Binary search works only when the input vector is sorted in ascending order.
  ///
  /// This function chooses the middle element then selects the left or right half
  /// based on comparing the target value with the middle value.  
  /// It does this recursively until the value is found or the search space is empty.
  ///
  /// Time complexity:
  /// - Best case: O(1)
  /// - Worst case: O(log n)
  ///
  /// Returns:
  /// - Some(value) if the target exists
  /// - None if the target is not found
  ///
  /// Note:
  /// This implementation allocates new vectors in each recursive step
  /// because it slices and converts slices to Vec.  
  /// This is simple but not the most efficient way to implement binary search.
  ///
  /// Example:
  /// ```rust
  /// use utils::searcher::Searcher;
  ///
  /// let data = vec![1, 2, 3, 4, 5];
  ///
  /// assert_eq!(Searcher::<u32>::binary_search(&data, 1), Some(1));
  /// assert_eq!(Searcher::<u32>::binary_search(&data, 0), None);
  /// ```
  ///
  pub fn binary_search(data: &[T], value: T) -> Option<T> {
    if data.is_empty() {
      return None;
    }

    let mid = data.len() / 2;

    if data[mid] == value {
      Some(data[mid])
    } else if data[mid] > value {
      // search in left half
      Self::binary_search(&data[..mid], value)
    } else {
      // search in right half
      Self::binary_search(&data[mid..], value)
    }
  }
}
