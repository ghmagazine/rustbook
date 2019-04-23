fn main() {
    cc::Build::new()
        .file("c_src/ownership.c")
        .compile("ownership");
}

