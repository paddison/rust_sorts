pub fn sort<T>(unsorted: &mut Vec<T>) 
where T: Eq + Ord
{
    for _ in 0..unsorted.len() - 1 {
        for j in 0..unsorted.len() - 1 {
            if unsorted.get(j).ge(&unsorted.get(j + 1)) {
                unsorted.swap(j, j + 1);
            }
        }
    }
}