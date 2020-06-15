

#[allow(dead_code)]
pub fn sample(){
    println!("ch05_02_vec.rs");
}


/// ベクタは配列を表現する型,プリミティブ型の配列に似ているが以下の点が異なる
/// 1. 要素の追加や削除ができる
/// 2. 配列は( Boxポインタを使わない限り )スタック領域に配置されるが,ベクタはヒープ領域にデータを置く
/// ベクタを利用するには vec![] マクロを利用するのが便利
#[allow(dead_code)]
fn sample_0() {
    println!("sample_0");

    let v1 = vec![false, true, false]; // Vec<bool> 型
    let v2 = vec![0.0, -1.0, 1.0, 0.5]; // Vec<f64> 型

    assert_eq!(v2.len(), 4); // v2ベクタの長さは4

    // 長さ100のベクタを作り,全要素をoi32で初期化する
    // (要素の型はcloneトレイトを実装していなければならない)
    let v3 = vec![0; 100]; // Vec<i32> 型
    assert_eq!(v3.len(), 100);

    // ベクタは入れ子にできる,この要素はそれぞれが異なっても構わない
    let v4 = vec![vec!['a', 'b', 'c'], vec!['d']]; // Vec<Vec<char>> 型
    assert_eq!(v4[0].len(), 3);
    assert_eq!(v4[1].len(), 1);

    // ベクタは同じ型の要素の並び,異なる型の要素は持てない
    //let v5 = vec![false, 'a'];
    // -> error[E0308]: mismatched types

    // ベクタには値を追加,削除する方法がいくつか存在する
    let mut v6 = vec!['a', 'b', 'c']; // Vec<char> 型
    v6.push('d');                         // 最後尾に値を追加
    v6.push('e');
    assert_eq!(v6, ['a', 'b', 'c', 'd', 'e']); // v6の現在の値

    assert_eq!(v6.pop(), Some('e')); // 最後尾から値を取り出し
    v6.insert(1, 'f'); // インデックス1の位置に要素を挿入
    assert_eq!(v6, ['a', 'f', 'b', 'c', 'd']); // v6の現在の値

    let mut v7 = vec!['g', 'h'];
    v6.append(&mut v7); // v6の最後尾にv7の要素を追加
    assert_eq!(v6, ['a', 'f', 'b', 'c', 'd', 'g', 'h' ]);
    assert_eq!(v7, []); // v7は空になった( 要素がv6へ移動した )

    let a8 = ['i', 'j']; // 配列 a8 を作成
    v6.extend_from_slice(&a8); // v6の最後尾にa8の全要素を追加
    assert_eq!(v6, ['a', 'f', 'b', 'c', 'd', 'g', 'h', 'i', 'j']);
    assert_eq!(a8, ['i', 'j']); // a8は変更なし( a8の要素がコピーされた )

    // 空のベクタを作成するときはnew()メソッドを使う
    // また,事前に大まかな要素数がわかってるときは with_capacity( 要素数 ) メソッドを利用するとよい
    

}


/*
#[cfg(test)]
mod test {

    use super::sample_0;

    #[test]
    fn test_0() {
        sample_0();
    }



}
*/





