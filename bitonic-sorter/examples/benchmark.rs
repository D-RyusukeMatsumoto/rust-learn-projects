use num_cpus;

use bitonic_sorter::SortOrder;

// 第3段階のsort関数を seq_sort という別名で使用する
use bitonic_sorter::third::sort as seq_sort;

// 第4段階のsort関数を par_sort という別名で使用する
use bitonic_sorter::forth::sort as par_sort;

use bitonic_sorter::utils::{ is_sorted_ascending, new_u32_vec };

use std::{env, f64};
use std::str::FromStr;
use std::time::Instant;


// エントリポイント
fn main()
{
    // 1つめのコマンドライン引数を文字列として取得する
    if let Some(n) = env::args().nth(1)
    {
        // 文字列型から u32型 への変換を試み、成功したら bits に束縛する
        // 失敗した場合、エラーを起こして終了
        let bits = u32::from_str(&n).expect("error parsing argument");
        // 順次ソートと並列ソートを実行する
        run_sorts(bits)
    }
    else
    {
        // コマンドライン引数が指定されていなかったらヘルプメッセージを出力し、ステータスコード 1 で終了する
        eprintln!(
            "Usage {} <number of element in bits>",
            env::args().nth(0).unwrap());
        std::process::exit(1);
    }
}


// 並列ソートと順次ソートの実行
fn run_sorts(
    bits: u32)
{
    // 指定されたビット数からデータの要素数を求める
    // 例 :
    // 28 ビット -> 要素数 268,435,456
    // 26 ビット -> 要素数 67,108,864
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

    // 順次ソートを実行し、処理にかかった時間を取得する
    let seq_duration = timed_sort(&seq_sort, len, "seq_sort");

    // 並列ソートを実行し、処理にかかった時間を取得する
    let par_duration = timed_sort(&par_sort, len, "par_sort");

    // 並列ソートが順次ソートに対して何倍速かったのかを表示
    println!("speed up: {:.2}x", seq_duration / par_duration);
}


// 時間計測
fn timed_sort<F>(
    sorter: &F,
    len: usize,
    name: &str) -> f64
        where F: Fn(&mut [u32], &SortOrder) -> Result<(), String>,
{
    // 要素数 len のu32型ベクタを生成する
    let mut x = new_u32_vec(len);

    // sorter関数を呼び出すことでソートを実行
    // かかった時間 ( dur ) を記録する
    let start = Instant::now();
    sorter(&mut x, &SortOrder::Ascending).expect("Failed to sort: ");
    let dur = start.elapsed();

    // ソートした要素数とかかった時間 (秒) 表示
    let nano_secs = dur.subsec_nanos() as f64 + dur.as_secs() as f64 * 1e9_f64;
    println!("{}: sorted {} integers in {} seconds",
    name,
    len,
    nano_secs / 1e9);

    // ソート結果が正しいか検証する
    assert!(is_sorted_ascending(&x));

    nano_secs
}