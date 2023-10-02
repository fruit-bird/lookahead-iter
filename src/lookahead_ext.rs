use std::slice::Iter;

use crate::look::Look;

pub trait LookAheadExt: Sized + Iterator {
    fn next_window<'a, const N: usize, T>(self) -> Look<Self, N>
    where
        T: 'a,
        Self: Sized + Iterator<Item = &'a T>,
    {
        Look::new(self)
    }
}

// impl<T> LookAheadExt for dyn Iterator<Item = T> where dyn Iterator<Item = T>: Sized {}
impl<T> LookAheadExt for Iter<'_, T> {}

#[test]
fn sanity_check() {
    let vec = (0..10).collect::<Vec<_>>();
    let it = vec.iter();

    for x @ [_a, _b] in it.next_window() {
        println!("{:?}", x);
        println!("{:?}", x.len());
    }
}
