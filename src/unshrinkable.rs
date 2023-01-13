use quickcheck::{Gen, Arbitrary};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Unshrinkable<T: Debug + Clone + 'static>(Option<T>);

impl<T: Debug + Clone> Unshrinkable<T> {
    pub fn new(x: T) -> Self {
        Unshrinkable(Some(x))
    }

    pub fn take(self) -> T {
        self.0.unwrap()
    }
}

impl<T: Debug + Clone> Arbitrary for Unshrinkable<T> {
    fn arbitrary(_: &mut Gen) -> Self {
        Unshrinkable(None)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::Arbitrary;
    use quickcheck_macros::*;

    #[quickcheck]
    fn unshrinkable(l: u8, x: usize) {
        let l: usize = l.into();
        let yss: Vec<_> = (0..l).map(|_| super::Unshrinkable::new(x)).collect();
        for ys in yss.shrink() {
            let ys: Vec<_> = ys.into_iter().map(|y| y.take()).collect();
            assert!(ys.iter().all(|y| *y == x));
        }
    }
}
