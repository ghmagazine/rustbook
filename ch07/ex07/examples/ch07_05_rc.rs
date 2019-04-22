#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

use std::rc::Rc;

fn main() {
    let mut rc1 = Rc::new(Child(1));  // Rcポインタ経由でChild値をヒープ領域に格納する
    // strong_countでこのChild値の参照カウント（共同所有者の数）が得られる
    println!("(a) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);
    {
        let rc2 = Rc::clone(&rc1);  // cloneで共同所有者を作る。参照カウントが増える
        println!(
            "(b) count: {}, rc1: {:?}, rc2: {:?}",
            Rc::strong_count(&rc1), rc1, rc2
        );
    }  // rc2がスコープを抜け、参照カウントが減る
    println!("(c) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    // 参照カウントが1のときは可変の参照が得られる。そうでないときはNoneが返る
    if let Some(child) = Rc::get_mut(&mut rc1) {
        child.0 += 1;
    }
    println!("(d) count: {}, rc1: {:?}", Rc::strong_count(&rc1), rc1);

    let weak = Rc::downgrade(&rc1);  // Rc::downgradeでWeakポインタが得られる
    println!(
        "(e) count: {}, rc1: {:?}, weak: {:?}",
        Rc::strong_count(&rc1),  // 参照カウントは1。Weakポインタはカウントされない
        rc1,
        weak,
    );

    // WeakをRcにアップグレードするとChild値にアクセスできる
    if let Some(rc3) = weak.upgrade() {
        println!(
            "(f) count: {}, rc1: {:?}, rc3: {:?}",
            Rc::strong_count(&rc1),
            rc1,
            rc3,
        );
    }

    // rc1をドロップする（スコープを抜けたのと同じ） 参照カウントが0になりChildは破棄される
    std::mem::drop(rc1);
    println!("(g) count: 0, weak.upgrade(): {:?}", weak.upgrade());
}
