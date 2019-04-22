#[derive(Debug)]
struct Parent(usize, Child, Child);

use std::ops::Drop;

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    println!("Move Semantics:");
    move_semantics();
    println!("\nBorrow:");
    borrow();
}

fn move_semantics() {
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;                // 値の所有権をp1からp2にムーブする
    println!("p2: {:?}", p2);
    // println!("p1: {:?}", &p1); // p1は値の所有権を失ったためアクセス不可
    // → error[E0382]: borrow of moved value: `p1`

    p1 = Parent(2, Child(21), Child(22)); // p1を別の値に束縛する
    println!("p1: {:?}", p1); // p1は別の値の所有権を持つためアクセスできる
}

// Parentへの不変の参照を引数にとる
fn f1(p: &Parent) {
    println!("p:  {:?}", p);
}

// Parentへの可変の参照を引数にとる
fn f2(p: &mut Parent) {
    p.0 *= 1;
}

fn borrow() {
    let mut p1 = Parent(1, Child(11), Child(12));
    f1(&p1);                   // f1には所有権をムーブせず、不変の参照を渡す
    f2(&mut p1);               // f2には所有権をムーブせず、可変の参照を渡す
    println!("p1: {:?}", p1);  // p1は値の所有権を失っていないのでアクセスできる
}
