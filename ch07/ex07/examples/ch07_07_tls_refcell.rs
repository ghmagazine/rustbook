use std::cell::RefCell;
use std::collections::HashSet;

thread_local!(
    // TLSに変数RABBITSを作成する。thread_localマクロはmutキーワードをサポートしない
    static RABBITS: RefCell<HashSet<&'static str>> = {
        // 初期化のコードはそのスレッドでRABBITSが初めてアクセスされたときに実行される
        // ここでは2要素のHashSetを作成し、可変にするためにRefCellで包んでいる
        let rb = ["ロップイヤー", "ダッチ"].iter().cloned().collect();
        RefCell::new(rb)
    }
);

fn main() {
    // TLSに置いた値にアクセスするにはwithを使う。mainスレッドのRABBITSが得られる
    RABBITS.with(|rb| {   // &RefCell<HashSet<'static str>>型
        assert!(rb.borrow().contains("ロップイヤー"));
        rb.borrow_mut().insert("ネザーランド・ドワーフ");  // 要素を追加
    });

    std::thread::spawn(||   // 別スレッドを起動し、そこでも要素を追加する
        RABBITS.with(|rb| rb.borrow_mut().insert("ドワーフホト"))
    ).join().expect("Thread error");   // スレッドの終了を待つ

    // mainスレッドのRABBITSにアクセスする
    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ネザーランド・ドワーフ"));
        // RABBITSはスレッドごとに持つので、別スレッドで追加した要素はここにはない
        assert!(!rb.borrow().contains("ドワーフホト"));
    });
}
