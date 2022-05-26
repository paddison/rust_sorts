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

pub mod iterative {
    use std::fmt::Debug;

    pub fn sort<T>(unsorted: &mut [T]) 
    where T: PartialEq + Copy + PartialOrd + Debug
    {
        let low = 0;
        let high = unsorted.len();

        // simulate the call stack of functions in order to store partition sizes
        let mut stack = vec![];

        stack.push(low);
        stack.push(high);

        while stack.len() > 0 {
            let high = stack.pop().unwrap();
            let low = stack.pop().unwrap();
            let p = partition(&mut unsorted[low..high]) + low;

            // if there are elements on left side, push bounds on stack
            if p > low {
                stack.push(low);
                stack.push(p);
            }

            // if there are elements on right side
            if p + 1 < high {
                stack.push(p + 1);
                stack.push(high);
            }
        }
    }

    fn partition<T>(arr: &mut[T]) -> usize 
    where T: PartialEq + Copy + PartialOrd + Debug
    {
        let len = arr.len();
        if len <= 1 {
            return 0;
        }

        let pivot = arr[len - 1];

        // indices for to swap elements
        let mut l_i = 0;
        let mut r_i = len - 2;

        loop {
            while arr[l_i] < pivot && l_i < r_i {
                l_i += 1;
            }
            while arr[r_i] > pivot && l_i < r_i {
                r_i -= 1;
            }

            if r_i == l_i {
                if arr[l_i] >= pivot {
                    // if pivot is not largest element, return index of last swapped element
                    arr.swap(l_i, len - 1);
                    break l_i;
                } else {
                    // if pivot was largest element, return index of pivot
                    break len - 1;
                }
            } else {
                arr.swap(l_i, r_i);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use rand::thread_rng;
        use rand::seq::SliceRandom;
        use super::*;
        
        #[test]
        fn test_quick_iter_n() {
            let n = 1000; //change me
            let mut unsorted = vec![];
            let mut expected = vec![];
            let mut rng = thread_rng();
            for i in 0..n {
                unsorted.push(i);
                expected.push(i);
            }
            unsorted.shuffle(&mut rng);
            sort(&mut unsorted);
            assert_eq!(unsorted, expected);
        }
    }
}