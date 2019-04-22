#[allow(dead_code)]
static I0: i32 = 42;           // static変数。'staticスコープを持つ

// この関数は'staticライフタイムを持つ任意の型を引数にとる
fn take_static<T: 'static>(_x: T) { }

fn main() {
    #[allow(unused_mut)]
    let mut s0: &'static str;
    let s1 = "42";             // &str型。文字列リテラル（データは静的領域にある）
    let s2 = 42.to_string();   // String型（データはヒープ領域にある）
    s0 = s1;                   // 文字列リテラルへの参照は'staticライフタイムを持つ
    // s0 = &s2;               // コンパイルエラー。String型から&'static strは作れない
    // → error[E0597]: `s2` does not live long enough
    
    println!("{}", s0);

    take_static(s1);           // &'static str型。OK
    // take_static(&s2);       // &String型。コンパイルエラー（'static要求を満たせない）
    take_static(s2);           // String型。OK
}
