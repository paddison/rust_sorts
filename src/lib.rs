//! # sorts
//! 
//! Implementation of some sorting algorithms. 
//! As of writing this, only merge sort is implemented.

use std::slice::Iter;

/// Sorts a given vector with the merge sort algorithm
/// 
/// # Examples
/// 
/// ```
/// use sorts::merge_sort;
/// let unsorted = vec![2, 1, 3, 5];
/// let sorted = merge_sort(&unsorted);
/// 
/// assert_eq!(sorted, vec![1, 2, 3, 5])
/// ```
pub fn merge_sort(input: &Vec<i32>) -> Vec<i32> {
    let unsorted = input.clone();
    merge_sort_helper(unsorted)

}

/// A helper function which splits the vector and calls the merge function.
fn merge_sort_helper(mut unsorted: Vec<i32>) -> Vec<i32> {
    if unsorted.len() < 2 {
        return unsorted
    }
    let left: Vec<i32> = unsorted.drain(..unsorted.len() / 2).collect();
    let right: Vec<i32> = unsorted.drain(..).collect();

    merge(merge_sort_helper(left).iter(), 
    merge_sort_helper(right).iter())

}

/// Merges two given iterables together, with respect to the size of their elements.
fn merge(mut left_iter: Iter<'_, i32>, mut right_iter: Iter<'_, i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut left = left_iter.next();
    let mut right = right_iter.next();

    loop {
        if left.is_none() && right.is_none() {
            return result;
        } else if left.is_none() || (right.le(&left) && right.is_some()) {
            result.push(*right.unwrap());
            right = right_iter.next();
        } else {
            result.push(*left.unwrap());
            left = left_iter.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_number_vector() {
        let sorted = merge_sort(&vec![2, 1, 3, 5]);
        assert_eq!(sorted, vec![1, 2, 3, 5]);        
    }
}