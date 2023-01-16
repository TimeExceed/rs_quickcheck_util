use quickcheck::{Gen, Arbitrary};
use std::fmt::Debug;

/// Unshrinkable suppresses element-shrinking of quickcheck.
///
/// For example, for `Vec<T>`'s, sometimes we want to only shrink the size but not the elements.
/// Perhaps `T` is unshrinkable. Perhaps we do not care of what `T` instances are.
/// Anyway, this requirement can be fulfilled by the following:
///
/// ```rust
/// let ys: Vec<_> = xs.into_iter() // xs is of type Vec<T>
///     .map(|x| Unshrinkable::new(x))
///     .collect();
/// let it = ys.shrink()
///     .map(|ys| ys.into_iter().map(|y| y.take()).collect::<Vec<T>>());
/// Box::new(it)
/// ```
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
