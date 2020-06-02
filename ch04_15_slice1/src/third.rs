
// スライスに対する主な操作
pub fn sample(){
    println!("third.rs");

    // 不変のスライスと可変のスライスで共通の処理
    let a1: [i32; 0] = [];
    let s1 = &a1;         // 不変のスライスを作成
    assert!(s1.is_empty());       // 空のスライスかチェック
    assert_eq!(s1.len(), 0);      // 長さは0
    assert_eq!(s1.first(), None); // 最初の要素は存在しない

    let a2 = ["zero", "one", "two", "three", "four"];
    let s2 = &a2[1..4];           // 不変のスライスを作成
    assert!(!s2.is_empty());              // 空ではない
    assert_eq!(s2.len(), 3);              // 長さは3
    assert_eq!(s2.first(), Some(&"one")); // 最初の要素

    assert_eq!(s2[1], "two");                  // 2番目の要素
    // assert_eq!(s3[3], "?");                 // 4番目の要素,存在しないのでpanicする
    assert_eq!(s2.get(1), Some(&"two")); // 2番目の要素をえる別の方法
    assert_eq!(s2.get(3), None);         // 4番目の要素,存在しないのでNone

    assert!(s2.contains(&"two"));                 // "two"を要素に持つ
    assert!(s2.starts_with(&["one", "two"])); // "one", "two"で始まる
    assert!(s2.ends_with(&["two", "three"])); // "two", "three"で終わる

    // 可変のスライスだけ可能な操作
    let mut a3 = [6, 4, 2, 8, 0, 9, 4, 3, 7, 5, 1, 7];

    // 一部の要素を昇順にソートする
    &mut a3[2..6].sort();
    assert_eq!(&a3[2..6], &[0, 2, 8, 9]);

    // スライスを2つの可変スライスへ分割する
    let (s3a, s3b) = &mut a3.split_at_mut(5);

    // &mut を省略しても結果は同じ,型強制により自動的にスライスが作られる
    // こちらの記述のほうが一般的.
    //let (s3a, s3b) = a3.split_at_mut(5);

    // 前半を逆順にする
    s3a.reverse();
    assert_eq!(s3a, &[8, 2, 0, 4, 6]);

    // 後半を昇順にソートする
    s3b.sort_unstable();
    assert_eq!(s3b, &[1, 3, 4, 5, 7, 7, 9]);

    // sort() と sort_unstable() の違い
    // sort() は安定ソートなので同純名データのソート前の順序がソート後も保存される
    // sotr_unstable() は判定ソートではないが,一般的に sort() より高速

    // このサンプルのコードはごく一部なのでスライスのリファレンスを読み,より理解を深めると良き
}



