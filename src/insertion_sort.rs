pub fn sort<T: PartialOrd + Copy>(list: &mut Vec<T>) {
    for i in 1..list.len() {
        for j in (1..=i).rev() {
            if list[j] < list[j - 1] {
                list.swap(j, j - 1);
            }
        }    
    }
}