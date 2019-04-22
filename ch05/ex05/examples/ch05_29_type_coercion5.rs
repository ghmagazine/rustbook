fn main() {
    let v1: Vec<u8> = vec![3, 4, 5];
    assert_eq!(v1.first(), Some(&3u8));  // first()メソッドのレシーバはv1が束縛されたベクタ
}
