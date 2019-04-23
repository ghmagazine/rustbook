// trait Overload {
//     fn call(&self) -> &'static str;
// }

// impl Overload for i32 {
//     fn call(&self) -> &'static str {
//         "i32"
//     }
// }

// impl Overload for str {
//     fn call(&self) -> &'static str {
//         "str"
//     }
// }

// fn main() {
//     assert_eq!(Overload::name(&1i32), "i32");
//     assert_eq!(Overload::name("str"), "str");
// }

// fn main() {
//     assert_eq!(1i32.name(), "i32");
//     assert_eq!("str".name(), "str");
// }

trait Overload1<T> {
    fn call(&self, t: T) -> &'static str;
}

impl Overload1<i32> for i32 {
    fn call(&self, _: i32) -> &'static str {
        "(i32, i32)"
    }
}

impl Overload1<char> for i32 {
    fn call(&self, _: char) -> &'static str {
        "(i32, char)"
    }
}

fn main() {
    assert_eq!(1i32.call(2i32), "(i32, i32)");
    assert_eq!(1i32.call('c'), "(i32, char)");
}
