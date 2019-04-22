use num_cpus;

use bitonic_sorter::SortOrder;
// 第3段階のsort関数をseq_sortという別名で使用する
use bitonic_sorter::third::sort as seq_sort;
// 第4段階のsort関数をpar_sortという別名で使用する
use bitonic_sorter::fourth::sort as par_sort;
use bitonic_sorter::utils::{is_sorted_ascending, new_u32_vec};

use std::{env, f64};
use std::str::FromStr;
use std::time::Instant;

fn main() {
    // 1つ目のコマンドライン引数を文字列として取得する
    if let Some(n) = env::args().nth(1) {
        // 文字列型からu32型への変換を試み、成功したらbitsに束縛する
        // もし失敗したならエラーを起こして終了する
        let bits = u32::from_str(&n).expect("error parsing argument");
        // 順次ソートと並列ソートを実行する
        run_sorts(bits);
    } else {
        // コマンドライン引数が指定されてなかったらヘルプメッセージを表示して
        // ステータスコード1で終了する
        eprintln!(
            "Usage {} <number of elements in bits>",
            env::args().nth(0).unwrap()
        );
        std::process::exit(1);
    }
}

fn run_sorts(bits: u32) {
    // 指定されたビット数からデータの要素数を求める
    // 例：
    // 28ビット → 要素数 268,435,456
    // 26ビット → 要素数  67,108,864
    let len = 2.0_f64.powi(bits as i32) as usize;

    // ソートする要素数とデータの見積もりサイズを表示する
    println!(
        "sorting {} integers ({:.1} MB)",
        len,
        (len * std::mem::size_of::<u32>()) as f64 / 1024.0 / 1024.0
    );

    // プロセッサの物理コア数と論理コア数を表示する
    println!(
        "cpu info: {} physical cores, {} logical cores",
        num_cpus::get_physical(),
        num_cpus::get()
    );

    // 順次ソートを実行して、処理にかかった時間を得る
    let seq_duration = timed_sort(&seq_sort, len, "seq_sort");

    // 並列ソートを実行して、処理にかかった時間を得る
    let par_duration = timed_sort(&par_sort, len, "par_sort");

    // 並列ソートが順次ソートに対して何倍速かったのか表示する
    println!("speed up: {:.2}x", seq_duration / par_duration);
}

fn timed_sort<F>(sorter: &F, len: usize, name: &str) -> f64
where
    F: Fn(&mut [u32], &SortOrder) -> Result<(), String>,
{
    // 要素数lenのu32型ベクタを生成する
    let mut x = new_u32_vec(len);

    // sorter関数を呼び出すことで、ソートを実行する
    // かかった時間（dur）を記録する
    let start = Instant::now();
    sorter(&mut x[..], &SortOrder::Ascending).expect("Failed to sort: ");
    let dur = start.elapsed();

    // ソートした要素数とかかった時間（秒）を表示する
    let nano_secs = dur.subsec_nanos() as f64 + dur.as_secs() as f64 * 1e9_f64;
    println!(
        "{}: sorted {} integers in {} seconds",
        name,
        len,
        nano_secs / 1e9
    );

    // ソート結果が正しいか検証する
    assert!(is_sorted_ascending(&x[..]));

    nano_secs
}
