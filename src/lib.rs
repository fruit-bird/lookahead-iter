use std::marker::PhantomData;

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
    // type Item = [&'a <I as Iterator>::Item; N];
    type Item = Vec<<I as Iterator>::Item>;

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

#[test]
fn feature() {
    let v = (0..10).collect::<Vec<_>>();
    let mut lookahead_iter = LookAhead::<_, 6>::new(v.iter());

    while let Some(window) = lookahead_iter.next() {
        println!("{:?}", window);
    }
}
