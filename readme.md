# Collection of Sorting Algorithms

## Description

A small collection of 5 sorting algorithms. The Algorithms included are `Heap Sort`, `Merge Sort`, `Quick Sort`, `Bubble Sort`, `Insertion Sort`.

## Algorithms

### Heap Sort

Basic implementation, with some functions to get elements from the heap, or find the largest element.

### Merge Sort

Several implementations are included:
1. A basic recursive one, with no memory optimization
2. Recursive, with memory optimization, requiring 2N memory space
3. An iterative bottom up implementation, requiring 2N memory space

### Quick Sort

There are two implementations, one recursive, the other iterative

### Bubble Sort

Basic implementation, no optimizations.

### Insertion Sort

Basic implementation, no optimizations.

## Benchmark

It is possible to do some small benchmarks. Each benchmark will run with Heap, Quick, Merge and Insertion Sort, with a sorted dataset, an reversed dataset and a random dataset. 
The size of the datasets can be specified, by passing a `Vec<usize>` into the function.

Example:

```
use sorts::benchmark::benchmark_runner;
fn main() {
    benchmark_runner::run(5, vec![100, 500, 1000, 5000, 10000, 50000, 100000, 500000, 1000000]);
}
```