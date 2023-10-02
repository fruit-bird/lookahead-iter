#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct Look<I, const N: usize> {
    it: I,
}

impl<I, const N: usize> Look<I, N> {
    pub(crate) fn new(it: I) -> Look<I, N> {
        Look { it }
    }
}

impl<'a, I, const N: usize, T: 'a> Iterator for Look<I, N>
where
    I: Iterator<Item = &'a T>,
    T: Clone,
{
    type Item = [&'a T; N];

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next_chunk().ok()
    }
}
