pub fn target(n: usize) {
    let vec = Vec::<u8>::with_capacity(n);
    println!("{:p}", &vec);
}

#[test]
fn test() {
    target(0);
}
