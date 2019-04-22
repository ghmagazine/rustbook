fn main() {
    let t1 = ('a', 42);
    // let t2 = t1 as (u32, u8);
    // → error[E0605]: non-primitive cast: `(char, i32)` as `(u32, u8)`

    let v1 = vec![b'h', b'e', b'l', b'l', b'o']; // Vec<u8>型
    // let v2 = v1 as Vec<u16>;
    // → error[E0605]: non-primitive cast:
    //   `std::vec::Vec<u8>` as `std::vec::Vec<u16>`

    #[allow(unused_variables)]
    let t3 = (t1.0 as u32, t1.1 as u8);
    #[allow(unused_variables)]
    let v3 = v1.iter().map(|&n| n as u16).collect::<Vec<u16>>();

    // &str型はVec<u8>型への変換を対象としたFromトレイトを実装している
    let v4: Vec<u8> = From::from("hello");
    assert_eq!(v1, v4);
}
