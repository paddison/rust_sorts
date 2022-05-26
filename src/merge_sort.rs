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
pub mod unoptimized {
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

pub mod optimized {

    struct ArrayStruct {
        pub start: usize,
        pub length: usize,
    }

    pub fn sort<T>(input: &Vec<T>) -> Vec<T> 
    where T: Eq + Ord + Copy
    {
        let mut input = input.clone();
        let mut other = input.clone();
        let mut arr_struct = ArrayStruct { start: 0, length: input.len() };
        sort_helper(&mut input, &mut other, &mut arr_struct);
        other
    }

    fn sort_helper<'a, 'b, T>(input: &'a mut Vec<T>, target: &'a mut Vec<T>, arr_struct: &'b ArrayStruct) 
    where T: Eq + Ord + Copy
    {

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

    fn merge<'a, T>(input: &'a mut Vec<T>, target: &'a mut Vec<T>, left: ArrayStruct, right: ArrayStruct) 
    where T: Eq + Ord + Copy
    {
        let mut left_index = 0;
        let mut right_index = 0;
        let mut actual_index = 0;
        
        while left_index < left.length && right_index < right.length {
            if input[left_index + left.start] < input[right_index + right.start] {
                target[actual_index + left.start] = input[left_index + left.start];
                left_index += 1;
            } else {
                target[actual_index + left.start] = input[right_index + right.start];
                right_index += 1;
            }
            actual_index += 1;
        }

        if left_index == left.length {
            while right_index < right.length {
                target[actual_index + left.start] = input[right_index + right.start];
                right_index += 1;
                actual_index += 1;
            }
        } else {
            while left_index < left.length {
                target[actual_index + left.start] = input[left_index + left.start];
                left_index += 1;
                actual_index += 1;
            }
        }
    }
}

pub mod bottom_up {
    pub fn sort<T>(mut input: Vec<T>) -> Vec<T> 
    where T: Eq + Ord + Copy
    {
        let mut tmp = input.clone();
        merge_sort(&mut input, &mut tmp);
        input
    }

    fn merge_sort<T>(arr: &mut [T], tmp: &mut [T]) 
    where T: PartialOrd + Copy
    {
        
        let mut sub_len = 1; // length of each sub array to be sorted
        let len = arr.len();
        // sub len = 1, 2, 4, 8, ...
        while sub_len < len {
            // create bounds for each sub array  
            // 0:1 + 1:2, ...
            // 0:2 + 2:4, ...
            let mut sub_start = 0; // start of subarray (sub_len = 1, sub_start = 0, 2, 4, sub_len = 2, sub_start = 0, 4, 8 etc.)
            while sub_start < len - sub_len {
                let sub_mid = sub_start + sub_len; // where to split each sub array
                let sub_end = if sub_mid + sub_len < len { sub_mid + sub_len } else { len };
                merge(&tmp[sub_start..sub_mid], &tmp[sub_mid..sub_end], &mut arr[sub_start..sub_end]);
                // copy sorted elements from arr to tmp
                for (i, ele) in arr[sub_start..sub_end].iter().enumerate() {
                    tmp[sub_start + i] = *ele;
                }
                sub_start += 2 * sub_len;
            }
            sub_len <<= 1;
        }
    }

    fn merge<T>(left: &[T], right: &[T], arr: &mut [T]) 
    where T: PartialOrd + Copy
    {
        let mut l_i = 0;
        let mut r_i = 0;
        for i in 0..arr.len() {
            match (left.get(l_i), right.get(r_i)) {
                (Some(l), Some(r)) => {
                    if l < r {
                        arr[i] = *l;
                        l_i += 1;
                    } else {
                        arr[i] = *r;
                        r_i += 1;
                    }
                },
                (Some(l), None) => {
                    arr[i] = *l;
                    l_i += 1;
                },
                (None, Some(r)) => { 
                    arr[i] = *r;
                    r_i += 1;
                },
                (None, None) => panic!("Overindexed both arrays, verify array sizes") // should actually never happen
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bottom_up;
    use rand::thread_rng;
    use rand::seq::SliceRandom;
    #[test]
    fn test_bottom_up_8_ele() {
        let unsorted = vec![2, 4, 9, 3, 6, 4, 7, 1];
        let sorted = bottom_up::sort(unsorted);
        assert_eq!(sorted, vec![1, 2, 3, 4, 4, 6, 7, 9]);
    }

    #[test]
    fn test_bottom_up_n_ele() {
        let n = 123; //change me
        let mut unsorted = vec![];
        let mut expected = vec![];
        let mut rng = thread_rng();
        for i in 0..n {
            unsorted.push(i);
            expected.push(i);
        }
        unsorted.shuffle(&mut rng);
        println!("{:?}", unsorted);
        let sorted = bottom_up::sort(unsorted);
        assert_eq!(sorted, expected);
        println!("{:?}", sorted);
    }
}