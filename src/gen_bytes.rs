use quickcheck::Gen;

pub fn gen_bytes<R>(
    g: &mut Gen,
    alphabet: &[u8],
    stopper: u8,
    len_range: R,
) -> Vec<u8>
where R: std::ops::RangeBounds<usize>
{
    let mut res = vec![];
    let min_len: usize = match len_range.start_bound() {
        std::ops::Bound::Unbounded => 0,
        std::ops::Bound::Included(n) => *n,
        std::ops::Bound::Excluded(n) => n + 1,
    };
    while res.len() < min_len {
        let ch = *g.choose(alphabet).unwrap();
        if ch != stopper {
            res.push(ch);
        }
    }
    let max_len: Option<usize> = match len_range.end_bound() {
        std::ops::Bound::Unbounded => None,
        std::ops::Bound::Included(n) => Some(n + 1),
        std::ops::Bound::Excluded(n) => Some(*n),
    };
    loop {
        let ch = *g.choose(alphabet).unwrap();
        if ch == stopper {
            break;
        }
        match max_len {
            Some(n) if res.len() + 1 >= n => {
                break;
            }
            _ => {}
        }
        res.push(ch);
    }
    res
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::*;

    #[quickcheck]
    fn both_included(a: u8, b: u8) {
        const STOPPER: u8 = b'.';
        const ALPHABET: &[u8] = b"abcd.";
        let (a, b) = if a < b {
            (a as usize, b as usize)
        } else {
            (b as usize, a as usize)
        };
        let mut g = quickcheck::Gen::new(32);
        let xs = super::gen_bytes(&mut g, ALPHABET, STOPPER, a..=b);
        assert!(xs.len() >= a, "left={}, right={}", xs.len(), a);
        assert!(xs.len() <= b, "left={}, right={}", xs.len(), b);
        assert!(xs.iter().all(|x| ALPHABET.contains(x)));
        assert!(xs.iter().all(|x| *x != STOPPER));
    }

    #[quickcheck]
    fn left_included_right_excluded(a: u8, b: u8) {
        const STOPPER: u8 = b'.';
        const ALPHABET: &[u8] = b"abcd.";
        let (a, b) = match (a, b) {
            (a, b) if a < b => (a as usize, b as usize),
            (a, b) if a > b => (b as usize, a as usize),
            _ => (a as usize, (b as usize) + 1),
        };
        let mut g = quickcheck::Gen::new(32);
        let xs = super::gen_bytes(&mut g, ALPHABET, STOPPER, a..b);
        assert!(xs.len() >= a, "left={}, right={}", xs.len(), a);
        assert!(xs.len() < b, "left={}, right={}", xs.len(), b);
        assert!(xs.iter().all(|x| ALPHABET.contains(x)));
        assert!(xs.iter().all(|x| *x != STOPPER));
    }
}
