
// スライスに対する主な操作
pub fn sample(){
    println!("third.rs");

    let a1: [i32; 0] = [];
    let s1 = &a1;         // 不変のスライスを作成
    assert!(s1.is_empty());       // 空のスライスかチェック
    assert_eq!(s1.len(), 0);      // 長さは0
    assert_eq!(s1.first(), None); // 最初の要素は存在しない

    // まだ途中




}



