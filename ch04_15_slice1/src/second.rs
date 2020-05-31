

// イミュータブルなスライスとミュータブルなスライス
#[allow(dead_code)]
pub fn sample() {
    println!("second.rs");

    let  mut a1 = [5, 4, 3, 2]; // 配列 [i32; 4]型
    let s1 = &mut a1[1..3]; // 可変のスライス, &mut[i32]型

    s1[0] = 6;               // スライスの最初の要素を6にする
    s1[1] *= 10;             // 2番目の要素を10倍する
    s1.swap(0, 1);     // 要素を交換
    assert_eq!(s1, [30, 6]); // スライスの内容を確認

    // 参照元の配列の内容を確認
    assert_eq!(a1, [5, 30, 6, 2]); // スライスを通じて配列の内容が変更された
}


// テストの作成
#[cfg(test)]
mod tests {
    use super::sample;

    #[test]
    fn test1() {
        sample();
    }

}