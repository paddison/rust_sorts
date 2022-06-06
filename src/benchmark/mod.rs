// Create datasets of varying sizes
// from 10^1 to 10^7 (need to check if feasible)
// three types of datasets:
// 1. sorted
// 2. reversed
// 3. random

// benchmarking always takes the raw time from when the sort starts till the end
// results should be written to a csv file, for each algorithm in the form of
// datatype size1 size2 size3 size4 .... where sizes are averaged
// sort
// rev
// rand
pub mod dataset_gen;
pub mod benchmark_runner;