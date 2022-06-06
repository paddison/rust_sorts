use std::fmt::Display;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::time::Instant;
use chrono::prelude::*;

use crate::heap_sort;
use crate::merge_sort;
use crate::quick_sort;
use crate::insertion_sort;
use super::dataset_gen;
use super::dataset_gen::DataType;

// run each benchmark individualy for sorting algorithm
// store results to a file
// one average, one individual runs

#[derive(Clone, Copy)]
enum Alg {
    Heap,
    Merge,
    Quick,
    Insert,
}

impl Display for Alg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alg::Heap => write!(f, "heap"),
            Alg::Merge => write!(f, "merge"),
            Alg::Quick => write!(f, "quick"),
            Alg::Insert => write!(f, "insert"),
        }
    }
}

pub fn run(repeats: usize, data_sizes: Vec<usize>) {
    let algs = vec![
        Alg::Heap, 
        Alg::Merge, 
        Alg::Quick, 
        Alg::Insert
    ];
    let data_types = vec![DataType::Sort, DataType::Rev, DataType::Rand];
    let time_stamp = Local::now();
    println!("Starting...");

    for alg in algs {
        println!("Doing {}...", alg);

        let path_string = format!("data/{}_avg_{}_{}.csv", alg, time_stamp.date(), time_stamp.time());
        let mut writer = BufWriter::new(File::create(&path_string).unwrap());
        for size in &data_sizes {
            let _ = writer.write(format!("{size},").as_bytes());
        }
        let _ = writer.write(b"\n");
        
        for data_type in &data_types {
            println!("Data type: {:?}", data_type);

            // results for a single row in the file
            let mut results = vec![];

            for size in &data_sizes {
                // println!("Size of data: {}", size);
                // results for a specific size
                let mut size_results = vec![];

                for _ in 0..repeats {
                    let data = dataset_gen::create_dataset(*size, *data_type);
                    size_results.push(do_benchmark(data, alg));
                }

                let avg_time = size_results.iter().sum::<u128>() / repeats as u128;
                results.push(avg_time);
            }

            for result in results {
                let _ = writer.write(format!("{result},").as_bytes());
            }
            let _ = writer.write(b"\n");
        }
        let _ = writer.flush();
    }
}

fn do_benchmark(mut data: Vec<usize>, alg: Alg) -> u128 {
    match alg {
        Alg::Heap => { 
            let start = Instant::now();
            heap_sort::sort(&mut data);
            start.elapsed()
        },
        Alg::Merge => {
            let start = Instant::now();
            merge_sort::bottom_up::sort(data);
            start.elapsed()
        }
        Alg::Quick => {
            let start = Instant::now();
            quick_sort::iterative::sort(&mut data);
            start.elapsed()
        }
        Alg::Insert => {
            let start = Instant::now();
            insertion_sort::sort(&mut data);
            start.elapsed()
        }
    }.as_nanos()
    
}

