#[derive(Default)]
struct A {
    f0: u8,
    f1: u32,
    f2: u8,
}

fn main() {
    let a: A = Default::default();
    println!(
        "struct A ({} bytes)\n  f0: {:p}\n  f1: {:p}\n  f2: {:p}\n",
        std::mem::size_of::<A>(),
        &a.f0,
        &a.f1,
        &a.f2
    );
}
