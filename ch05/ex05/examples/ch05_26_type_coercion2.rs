fn f1(n: &mut usize, str: &str, slice: &[i32]) {
    *n = str.len() + slice.len()
}

fn main() {
    let mut b1 = Box::new(0);  // Box<usize>型
    let s1 = String::from("deref");
    let v1 = vec![1, 2, 3];

    // Derefによる型強制が起こる：
    // - &mut Box<usize> → &mut usize
    // - &String → &str
    // - &Vec<i32> → &[i32]
    f1(&mut b1, &s1, &v1);
    assert_eq!(8, *b1);
}
