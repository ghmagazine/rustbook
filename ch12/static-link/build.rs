fn main() {
    cc::Build::new().file("c_src/fib.c").compile("fib");
}

