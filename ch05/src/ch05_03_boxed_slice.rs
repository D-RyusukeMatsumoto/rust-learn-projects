use test::assert_test_result;

pub fn sample(){
    println!("ch05_03_boxed_slice.rs");

    // 4要素のベクタVec<i32>を作り,要素を1つ足して5要素に拡張する
    let mut v1 = vec![0, 1, 2, 3];
    v1.push(4);
    println!("v1 len: {}, capacity: {}", v1.len(), v1.capacity());
    assert_eq!(v1.len(), 5);
    assert_eq!(v1.capacity(), 8);
    // -> [v1 len: 5, capacity: 8]と表示される,5要素だが8要素分のメモリを確保している

    // Box<[i32]>に変換する,余分なメモリを持たなくするためにVecのshrink_to_fit()メソッドが事項されてからBox化される
    let s1 = v1.into_boxed_slice();

    // 余分なメモリを持ってないことを確認するためにVec<i32>に戻す
    let v2 = s1.into_vec();
    println!("v2 len: {}, capacity: {}", v2.len(), v2.capacity());
    assert_eq!(v2.len(), 5);
    assert_eq!(v2.capacity(), 5);
    // -> [v2 len: 5, capacity: 5]と表示される,5要素ぴったりのメモリを確保していることがわかる
}


#[cfg(test)]
mod test {

    use super::sample;

    fn test(){
        //sample();
    }

}


