use super::SortOrder;
use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_order<T: Ord>(x: &[T], order: &SortOrder) -> bool {
    x.windows(2).all(|pair| match *order {
        SortOrder::Ascending => pair[0] <= pair[1],
        SortOrder::Descending => pair[0] >= pair[1],
    })
}
