// 戻り値の型を省略。コンパイラは戻り値がユニット型だと解釈する
fn hello() {
    println!("Hello");
}

fn main() {
    // 関数を呼び出し、（ないはずの）戻り値に変数retを束縛する
    let ret = hello();
    // アサーションでretの値がユニット値と等しいことを検査する
    assert_eq!(ret, ());

    // size_of::<型>()は、その型の値がメモリ上で占める大きさをバイト数で返す
    assert_eq!(std::mem::size_of::<()>(), 0);  // 0バイト
}
