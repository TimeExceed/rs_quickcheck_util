/// Shrinks one field of an object.
///
/// For any object `t` of type `T` and `a` is one of its field,
/// `shrink_a_field!(t, a)` will result in an iterator of type `T`,
/// whose `a` fields are shrinked and the other fields are kept untouched.
/// For example,
/// ```rust
/// use rs_quickcheck_util::shrink_a_field;
///
/// #[derive(Debug, Clone)]
/// struct T {
///     a: i64,
///     b: i64,
/// }
/// let t = T {
///     a: 100,
///     b: 42,
/// };
/// for x in shrink_a_field!(t, a) {
///     assert!(x.a < t.a);
///     assert_eq!(x.b, t.b);
/// }
/// ```
///
/// Sometimes, a field must be shrinked with a different behaviour,
/// e.g., for a Vec field, shrinking the vector but not shrinking their elements.
/// This can be achieved by "wrapping" the field.
///
/// ```rust
/// use rs_quickcheck_util::{shrink_a_field, Unshrinkable};
///
/// #[derive(Debug, Clone)]
/// struct T {
///     a: Vec<i64>,
/// }
/// let t = T {
///     a: [10, 10].into(),
/// };
/// let it = shrink_a_field!(
///     t,
///     a,
///     |xs: &Vec<i64>| {
///         xs.iter().map(|x| Unshrinkable::new(*x)).collect::<Vec<_>>()
///     },
///     |xs: Vec<Unshrinkable::<i64>>| {
///         xs.into_iter()
///             .map(|x| x.take())
///             .collect::<Vec<_>>()
///     }
/// );
/// for x in it {
///     assert!(x.a.iter().all(|y| *y == 10));
/// }
/// ```
///
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
    ($obj:expr, $field:ident, $wrap_fn:expr, $unwrap_fn:expr) => {
        {
            use quickcheck::Arbitrary;
            let wrap_fn = $wrap_fn;
            let unwrap_fn = $unwrap_fn;
            let xs = wrap_fn(&($obj.$field));
            let me = $obj.clone();
            xs.shrink()
                .map(move |x| {
                    let mut res = me.clone();
                    res.$field = unwrap_fn(x);
                    res
                })
        }
    }
}

/// Shrinks a mapping field whose key is determined by the value.
///
/// For a mapping field, sometimes the key is determined by the value.
/// To keep this constraint in shrinking, one can use `shrink_a_map_field`.
/// For example,
///
/// ```rust
/// use rs_quickcheck_util::shrink_a_map_field;
/// use std::collections::BTreeMap;
///
/// #[derive(Debug, Clone)]
/// struct T {
///     m: BTreeMap<String, usize>,
/// }
/// let t = T {
///     m: [
///         ("1".to_string(), 1),
///         ("2".to_string(), 2),
///         ("3".to_string(), 3),
///         ("4".to_string(), 4),
///         ("5".to_string(), 5),
///         ("6".to_string(), 6),
///     ].into(),
/// };
/// for x in shrink_a_map_field!(t, m, |x: &usize| -> String {format!("{}", x)}) {
///     for (k, v) in x.m.iter() {
///         assert_eq!(k.to_string(), format!("{}", v));
///     }
/// }
/// ```
/// Then, both size and values of `t.m` will be shrinked, but the relation between
/// keys and values are kept.
#[macro_export]
macro_rules! shrink_a_map_field {
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
        let bs: Vec<_> = shrink_a_map_field!(trial, b, |v: &usize| -> usize {*v}).collect();
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

    #[test]
    fn unshrinkable() {
        let z = A {
            a: 100,
            b: 100,
        };
        let it = shrink_a_field!(
            z,
            b,
            |x: &usize| {crate::Unshrinkable::new(*x)},
            |x: crate::Unshrinkable::<usize>| {x.take()}
        );
        for x in it {
            assert_eq!(x.b, z.b);
        }
    }

}
