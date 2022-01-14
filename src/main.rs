use sorts::heap;
fn main() {
    let mut heap = vec![11, 5, 8, 3, 4];
    let item = 15;
    heap::insert(&mut heap, item);
    println!("heap: {:?}", heap);

    let root = heap::extract(&mut heap);
    println!("heap: {:?}, root: {}", heap, root);
    let mut unsorted = vec![5,2,1,619,34,84,3,92,0,13,16,11,12,117,15];
    heap::sort(&mut unsorted);
    println!("{:?}", unsorted);
}