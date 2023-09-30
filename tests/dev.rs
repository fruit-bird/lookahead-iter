use lookahead_iter::LookAhead;

#[test]
fn feature() {
    let v = (0..10).collect::<Vec<_>>();
    let mut lookahead_iter = LookAhead::<_, 6>::new(v.iter());

    while let Some(window) = lookahead_iter.next() {
        println!("{:?}", window);
    }
}
