fn main() {
    let n1 = std::u8::MAX;  // u8型の最大値は255u8
    let n2 = 1u8;
    // 答えは256だがu8型では表現できない（オーバーフロー）
    let n3 = n1 + n2;
    println!("{}", n3);
}
