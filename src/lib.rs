//! # sorts
//! 
//! Implementation of some sorting algorithms. 
//! As of writing this, merge sort, quick sort, bubble sort, heap sort and insertion sort are implemented.

pub mod merge_sort;
pub mod quick_sort;
pub mod bubble_sort;
pub mod heap_sort;
pub mod insertion_sort;
pub mod benchmark;

#[cfg(test)]
mod tests {
    use super::merge_sort;
    use super::quick_sort;
    use super::bubble_sort;
    use super::insertion_sort;

    #[test]
    fn test_four_number_vector() {
        let sorted = merge_sort::optimized::sort(&vec![2, 1, 3, 5]);
        assert_eq!(sorted, vec![1, 2, 3, 5]);        
    }

    #[test]
    fn test_four_number_vector_quick() {
        let mut unsorted = vec![5, 550, 725, 883, 712, 633, 854, 162, 533, 654, 214, 413, 393, 383, 688, 960, 84];
        quick_sort::sort(&mut unsorted);
        assert_eq!(*unsorted, vec![5, 84, 162, 214, 383, 393, 413, 533, 550, 633, 654, 688, 712, 725, 854, 883, 960]);
    }

    #[test]
    fn test_four_number_vector_bubble() {
        let mut unsorted = vec!["a", "d", "c", "b"];
        bubble_sort::sort(&mut unsorted);
        assert_eq!(*unsorted, vec!["a", "b", "c", "d"])
    }

    #[test]
    fn insertion_sort() {
        let mut unsorted = vec![5, 3, 2, 6, 4, 5];
        insertion_sort::sort(&mut unsorted);
        assert_eq!(vec![2, 3, 4, 5, 5, 6], unsorted);
    }

    #[test]
    fn insertion_sort_long() {
        let unsorted = &mut vec![5, 550, 725, 883, 712, 633, 854, 162, 533, 654, 214, 413, 393, 383, 688, 960, 84];
        insertion_sort::sort(unsorted);
        assert_eq!(*unsorted, vec![5, 84, 162, 214, 383, 393, 413, 533, 550, 633, 654, 688, 712, 725, 854, 883, 960]);
    }
}