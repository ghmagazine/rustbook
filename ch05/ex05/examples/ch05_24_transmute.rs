fn main() {
    let p1 = Box::new(10);  // Box<i32>型

    // boxポインタを生ポインタ*mut i32型に変換したいが型キャストできない
    // let p2 = p1 as *mut i32;
    // → error[E0605]: non-primitive cast: ...

    // Boxポインタと*mutポインタはどちらも同じビット幅なのでtransmuteできる
    #[allow(unused_variables)]
    let p3: *mut i32 = unsafe { std::mem::transmute(p1) };

    let f1 = 5.6789e+3_f32;  // 5678.9

    // f32型からi32型へ型キャストする。小数点以下は切り捨てられる
    let i1 = f1 as i32;
    println!("{}", i1);   // 5678と表示される

    // f32型からi32型へtransmuteする
    let i2: i32 = unsafe { std::mem::transmute(f1) };
    println!("{}", i2);   // 浮動小数点数を整数として再解釈した値1169258291が表示される
}
