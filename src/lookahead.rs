use std::marker::PhantomData;

#[allow(unused)]
pub struct LookAhead<'a, I: Iterator, const N: usize> {
    iter: I,
    buffer: Vec<<I as Iterator>::Item>,
    _marker: PhantomData<&'a I>,
}

impl<'a, I: Iterator, const N: usize> LookAhead<'a, I, N> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            buffer: Vec::with_capacity(N),
            _marker: PhantomData,
        }
    }
}

impl<'a, I: Iterator, const N: usize> Iterator for LookAhead<'a, I, N>
where
    I::Item: Clone,
{
    // type Item = [&'a I::Item; N];
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.buffer.len() < N {
            if let Some(item) = self.iter.next() {
                self.buffer.push(item);
            } else {
                return None;
            }
        }

        let result = self.buffer.iter().cloned().collect();
        self.buffer.remove(0);

        Some(result)
    }
}
