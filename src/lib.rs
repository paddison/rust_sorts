//! # sorts
//! 
//! Implementation of some sorting algorithms. 
//! As of writing this, only merge sort is implemented.

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
pub mod merge_sort {
    use std::slice::Iter;

    pub fn sort(input: &Vec<i32>) -> Vec<i32> {
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
}

pub mod merge_sort_optimized {

    struct ArrayStruct {
        pub start: usize,
        pub length: usize,
    }

    pub fn sort(input: &Vec<i32>) -> Vec<i32> {
        let mut input = input.clone();
        let mut other = input.clone();
        let mut arr_struct = ArrayStruct { start: 0, length: input.len() };
        sort_helper(&mut input, &mut other, &mut arr_struct);
        other
    }

    fn sort_helper<'a, 'b>(input: &'a mut Vec<i32>, target: &'a mut Vec<i32>, arr_struct: &'b ArrayStruct) {

        let mut left = ArrayStruct { 
            start: arr_struct.start, 
            length: (arr_struct.length / 2), 
        };

        let mut right = ArrayStruct {
            start: arr_struct.length / 2 + arr_struct.start,
            length: arr_struct.length - arr_struct.length / 2,
        };

        if arr_struct.length > 2 {
            sort_helper(target, input, &mut left);
            sort_helper(target, input, &mut right);
        }
        merge(input, target, left, right)
    }

    fn merge<'a>(input: &'a mut Vec<i32>, target: &'a mut Vec<i32>, left: ArrayStruct, right: ArrayStruct) {
        let mut left_index = 0;
        let mut right_index = 0;
        let mut actual_index = 0;
        
        while left_index < left.length && right_index < right.length {
            if input.get(left_index + left.start) < input.get(right_index + right.start) {
                *target.get_mut(actual_index + left.start).unwrap() = *input.get(left_index + left.start).unwrap();
                left_index += 1;
            } else {
                *target.get_mut(actual_index + left.start).unwrap() = *input.get(right_index + right.start).unwrap();
                right_index += 1;
            }
            actual_index += 1;
        }

        if left_index == left.length {
            while right_index < right.length {
                *target.get_mut(actual_index + left.start).unwrap() = *input.get(right_index + right.start).unwrap();
                right_index += 1;
                actual_index += 1;
            }
        } else {
            while left_index < left.length {
                *target.get_mut(actual_index + left.start).unwrap() = *input.get(left_index + left.start).unwrap();
                left_index += 1;
                actual_index += 1;
            }
        }
    }
}

pub mod quick_sort {

    pub fn sort<T>(unsorted: &mut Vec<T>)
    where T: Eq + Ord,
    {
        let len = unsorted.len() - 1;
        partition(unsorted, 0, len);
    }

    fn partition<T>(unsorted: &mut Vec<T>, low: usize, high: usize)
    where T: Eq + Ord,
    {
        if low >= high {
            return
        }
        let mut l = low;
        let mut r = high - 1;

        loop {
            while unsorted.get(l).lt(&unsorted.get(high)) && l < r { // look for left element which is larger than pivot
                l += 1;
            }

            while unsorted.get(r).gt(&unsorted.get(high)) && l < r { // look for right element which is smaller  than pivot
                r -= 1;
            }

            if l == r {
                    if unsorted.get(l).ge(&unsorted.get(high)) { // make sure the last element is actually bigger than pivot and not just end of loop
                        unsorted.swap(l, high); // swap pivot
                    }
                break;
            } else {
                unsorted.swap(l, r); // swap both elements
            }
        }
        partition(unsorted, low, l);
        partition(unsorted, l + 1, high);
    }
}

pub mod bubble_sort {
    pub fn sort<T>(unsorted: &mut Vec<T>) 
    where T: Eq + Ord
    {
        for _ in 0..unsorted.len() - 1 {
            for j in 0..unsorted.len() - 1 {
                if unsorted.get(j).ge(&unsorted.get(j + 1)) {
                    unsorted.swap(j, j + 1);
                }
            }
        }
    }
}

pub mod heap {

    pub fn sort<T>(heap: &mut Vec<T>) 
    where T: Eq + Ord
    {
        let mut unsorted_len = heap.len();
        heapify(heap); // change the array into a valid heap

        while unsorted_len > 1 {
            heap.swap(0, unsorted_len - 1);     // swap the root element with the last element
            unsorted_len -= 1;                       // since the root element is the largest one, it is now sorted, meaning the unsorted part is one less
            move_down(heap, unsorted_len, 0);  // rebuild the heap
        }
    }

    pub fn move_down<T>(heap: &mut Vec<T>, len: usize, start: usize) 
    where T: Eq + Ord
    {
        let mut j = start;
        let mut largest = start;
        let mut left; 
        let mut right;
        loop {
            left = 2 * j + 1;
            right = 2 * j + 2;
            if left < len && heap[largest] < heap[left] {
                largest = left;
            }
            if right < len && heap[largest] < heap[right] {
                largest = right;
            }

            if j == largest {
                return
            } 

            heap.swap(j, largest);
            j = largest;
        }

    }

    pub fn heapify<T>(heap: &mut Vec<T>) 
    where T: Eq + Ord
    {
        let len = heap.len();
        for i in (0..len).rev() {
            move_down(heap, len, i);
        } 
    }

    
    // some heap functions which, are not used in the sort
    pub fn insert<T>(heap: &mut Vec<T>, item: T) 
    where T: Eq + Ord + Copy
    {
        let mut i = heap.len();
        // pushing item on heap
        heap.push(item);
        while i > 0 && item > heap[(i - 1) / 2] { // compare item to its parent node
            heap.swap(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }

    pub fn extract<T>(heap: &mut Vec<T>) -> T 
    where T: Eq + Ord
    {
        let len = heap.len();
        heap.swap(0, len - 1);
        let mut i = 0;
        while heap.get(i) < heap.get(2 * i + 1) || heap.get(i) < heap.get(2 * i + 2) {
            let largest = largest(heap.get(i), heap.get(2 * i + 1), heap.get(2 * i + 2), i);
            if largest < len {
                heap.swap(i, largest);
            }
            i = largest;
        }
        heap.pop().unwrap()
    }

    fn largest<T>(root: Option<&T>, left: Option<&T>, right: Option<&T>, i: usize) -> usize
    where T: Eq + Ord
    {
        if root > left && left > right {
            2 * i + 1
        } else {
            2 * i + 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::merge_sort;
    use super::quick_sort;
    use super::bubble_sort;

    #[test]
    fn test_four_number_vector() {
        let sorted = merge_sort::sort(&vec![2, 1, 3, 5]);
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
}