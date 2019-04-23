fn main() {
    let mut reg = {
        let s = "a(.*)b|[e-f]+".to_string();
        onigmo::Regex::new(&s).unwrap()
    };
    let s = "zzzzaffffffffb";
    match reg.search(s) {
        Some(ret) => {
            use std::str::from_utf8;
            for (beg, end) in ret.positions() {
                println!("{}", from_utf8(&s.as_bytes()[beg..end]).unwrap());
            }
        }
        None => println!("not match"),
    }
}
