use quickcheck::{Gen, Arbitrary};

pub fn shuffle<T>(g: &mut Gen, xs: &mut [T]) {
    let n = xs.len();
    for i in 0..n {
        let with = i + usize::arbitrary(g) % (n - i);
        xs.swap(i, with);
    }
}
