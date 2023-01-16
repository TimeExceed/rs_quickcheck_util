/// Shrinks one field of an object.
///
/// ```rust
/// struct T {
///     a: A,
///     b: B,
/// }
/// ```
/// Then, for an object `t: T`, `shrink_a_field!(t, a)` will result in an iterator of type `T`,
/// whose `a` field is shrinked but `b` field keeps the same as that of `t`.
///
/// For a mapping field, sometimes the key is determined by the value.
/// To keep this constraint in shrinking, one can use the 3-clause arm.
/// For example,
///
/// ```rust
/// struct T {
///     m: BTreeMap<String, usize>,
/// }
/// ```
/// For an object `t: T`,
/// ```rust
/// shrink_a_field!(t, m, |x: &usize| -> String {format!("{}", x)})
/// ```
/// Then, both size and values of `t.m` will be shrinked, but the relation between
/// keys and values is kept.
#[macro_export]
macro_rules! shrink_a_field {
    ($obj:expr, $field:ident) => {
        {
            use quickcheck::Arbitrary;
            let me = $obj.clone();
            me.clone().$field.shrink()
                .map(move |x| {
                    let mut res = me.clone();
                    res.$field = x;
                    res
                })
        }
    };
    ($obj:expr, $field:ident, $key_fn:expr) => {
        {
            use quickcheck::Arbitrary;
            let key_fn = $key_fn;
            let me = $obj.clone();
            let xs: Vec<_> = me.$field.values().cloned().collect();
            xs.shrink()
                .map(move |xs| {
                    let mut res = me.clone();
                    res.$field = xs.into_iter()
                        .map(|x| (key_fn(&x), x))
                        .collect();
                    res
                })
        }
    };
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::*;
    use std::collections::BTreeMap;

    #[quickcheck]
    fn shrink_simple_field(trial: A) {
        for x in shrink_a_field!(trial, a) {
            assert_eq!(x.b, trial.b);
            assert!(x.a < trial.a);
        }
    }

    #[derive(Debug, Clone)]
    struct A{a: usize, b: usize}

    impl quickcheck::Arbitrary for A {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self{
                a: usize::arbitrary(g),
                b: usize::arbitrary(g),
            }
        }
    }


    #[quickcheck]
    fn shrink_map(trial: B) {
        let bs: Vec<_> = shrink_a_field!(trial, b, |v: &usize| -> usize {*v}).collect();
        for x in bs.iter() {
            let keys: Vec<_> = x.b.keys().copied().collect();
            let values: Vec<_> = x.b.values().copied().collect();
            assert_eq!(keys, values);
            assert!(x.b.len() <= trial.b.len());
        }
        if !trial.b.is_empty() {
            assert!(bs.iter().any(|x| x.b.is_empty()));
        }
    }

    #[derive(Debug, Clone)]
    struct B{
        b: BTreeMap<usize, usize>,
    }

    impl quickcheck::Arbitrary for B {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let b = crate::gen_bytes(g, b"abc.", b'.', 0..).iter()
                .map(|_| {
                    let v = usize::arbitrary(g);
                    (v, v)
                })
                .collect();
            Self{b}
        }
    }
}
