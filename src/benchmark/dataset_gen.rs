use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy, Debug)]
pub enum DataType {
    Sort,
    Rev,
    Rand,
}

pub fn create_dataset(size: usize, data_type: DataType) -> Vec<usize> {
    match data_type {
        DataType::Sort => create_sorted(size),
        DataType::Rev => create_reversed(size),
        DataType::Rand => create_random(size),
    }
}

fn create_sorted(size: usize) -> Vec<usize> {
    let mut data = vec![];
    for i in 0..size {
        data.push(i);
    }

    data
}

fn create_reversed(size: usize) -> Vec<usize> {
    let mut data = vec![];
    for i in (0..size).rev() {
        data.push(i);
    }

    data
}

fn create_random(size: usize) -> Vec<usize> {
    let mut data = vec![];
    for i in 0..size {
        data.push(i);
    }
    let mut rng = thread_rng();
    data.shuffle(&mut rng);

    data
}

