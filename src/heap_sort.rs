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