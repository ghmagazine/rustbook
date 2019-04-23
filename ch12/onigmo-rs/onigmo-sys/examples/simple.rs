use onigmo_sys::*;
use std::mem;
use std::str::from_utf8_unchecked;

fn main() {
    unsafe {
        // 正規表現のパターン文字列
        let pattern = b"a(.*)b|[e-f]+";
        // マッチ対象です
        let s = b"zzzzaffffffffb";
        // `onig_new_without_alloc`で初期化するメモリをスタックに確保する。
        let mut reg: regex_t = mem::uninitialized();
        let mut einfo: OnigErrorInfo = mem::uninitialized();
        // 正規表現文字列をコンパイルし、`reg`に格納する。
        let r = onig_new_without_alloc(
            &mut reg as *mut _,
            // パターン文字列の先頭
            pattern as *const OnigUChar,
            // パターン文字列の末尾
            (pattern as *const OnigUChar).offset(pattern.len() as isize),
            // 今回、オプションは付けない
            ONIG_OPTION_NONE,
            // Rustの文字列はUTF-8エンコーディング
            &OnigEncodingUTF_8,
            // Onigmoのデフォルトの構文を使う
            OnigDefaultSyntax,
            &mut einfo,
        );
        // コンパイル結果の返り値が正常値でなければエラー
        if (r as ::std::os::raw::c_uint) != ONIG_NORMAL {
            // エラー情報を取得し出力 する
            let s: &mut [OnigUChar] = &mut [0; ONIG_MAX_ERROR_MESSAGE_LEN as usize];
            onig_error_code_to_str(s as *mut _ as *mut _, r as OnigPosition, &einfo);
            println!("ERROR: {}\n", from_utf8_unchecked(s));
            // 正規表現のエラーならそのまま終了
            return;
        }

        // マッチ情報を表わすデータを準備する
        let region = onig_region_new();

        // マッチ対象文字列の終端
        let end = (s as *const OnigUChar).offset(s.len() as isize);
        // マッチ開始位置
        let start = s as *const _;
        // マッチ終了位置
        let range = end;
        // 正規表現でマッチする
        let mut r = onig_search(
            &mut reg,
            s as *const _,
            end,
            start,
            range,
            region,
            ONIG_OPTION_NONE,
        );
        if 0 <= r {
            // 返り値が0以上ならマッチ成功
            println!("match at {}", r);
            let region = region.as_ref().unwrap();
            // グルーピングされた部分正規表現毎にマッチ位置を表示する
            for i in 0..(region.num_regs) {
                println!(
                    "{}: ({}-{})",
                    i,
                    *region.beg.offset(i as isize),
                    *region.end.offset(i as isize)
                );
            }
            r = 0;
        } else if (r as ::std::os::raw::c_int) == ONIG_MISMATCH {
            // 返り値が`ONIG_MISMATCH`なら正規表現とマッチ失敗
            println!("search fail");
            r = -1;
        } else {
            // それ以外ではOnigmoの内部エラー
            let s: &mut [OnigUChar] = &mut [0; ONIG_MAX_ERROR_MESSAGE_LEN as usize];
            onig_error_code_to_str(s as *mut _ as *mut _, r as OnigPosition, &einfo);
            println!("ERROR: {}\n", from_utf8_unchecked(s));
            std::process::exit(-1);
        }
        // 使ったリソースを手動で解放する
        onig_region_free(region, 1);
        onig_free_body(&mut reg);
        onig_end();
        std::process::exit(r as i32);
    }
}

