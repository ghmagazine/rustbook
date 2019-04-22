fn main() {
    let c1 = 'A';         // char型
    let c1_ptr = &c1;     // &char型。不変の参照（イミュータブルな参照）
    assert_eq!(*c1_ptr, 'A');

    let mut n1 = 0;       // i32型
    let n1_ptr = &mut n1; // &mut i32型。可変の参照（ミュータブルな参照）
    assert_eq!(*n1_ptr, 0);

    // 可変の参照では参照先の値を変更できる
    *n1_ptr = 1_000;
    assert_eq!(*n1_ptr, 1_000);
}
