use std::env;
use std::fs::File;
use std::io::BufReader;

// libクレートに分離したものを使う
use bicycle_book_wordcount::count;

fn main() {
    // 1. コマンドラインで指定された引数を読み込む
    let filename = env::args().nth(1).expect("1 argument FILENAME required");
    // 2. 指定されたファイルを開く
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    // 3. ファイルから1行ずつ読み込む
    // 第2引数 `Default::default` を加える
    let freqs = count(reader, Default::default());
    println!("{:?}", freqs);
}
