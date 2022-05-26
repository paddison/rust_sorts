use sorts::benchmark::benchmark_runner;
fn main() {
    // let mut heap = vec![11, 5, 8, 3, 4];
    // let item = 15;
    // heap_sort::insert(&mut heap, item);
    // println!("heap: {:?}", heap);

    // let root = heap_sort::extract(&mut heap);
    // println!("heap: {:?}, root: {}", heap, root);
    // let mut unsorted = vec![5,2,1,619,34,84,3,92,0,0,13,16,11,12,117,15];
    // heap_sort::sort(&mut unsorted);
    // println!("{:?}", unsorted);
    benchmark_runner::run(5, vec![100, 500, 1000, 5000, 10000, 50000, 100000, 500000]);
}