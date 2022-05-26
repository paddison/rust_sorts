

pub fn sort<T: PartialOrd + Copy>(list: &mut Vec<T>) {
    for i in 1..list.len() {
        let mut j = (i - 1) as isize;
        let cur = list[i];
        while j >= 0 && list[j as usize] > cur {
            list[(j + 1) as usize] = list[j as usize];
            j -= 1;
        }
        list[(j + 1) as usize] = cur;
    }
}

#[test]
fn test_n() {
    use rand::{thread_rng, prelude::SliceRandom};

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
        sort(&mut unsorted);
        assert_eq!(unsorted, expected);
        println!("{:?}", unsorted);
}