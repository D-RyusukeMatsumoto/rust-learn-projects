use super::SortOrder;
use std::cmp::Ordering;


pub fn sort_by<T, F>(
    x: &mut[T],
    comparator: &F) -> Result<(), String> where F: Fn(&T, &T) -> Ordering
{
    if x.len().is_power_of_two()
    {
        do_sort(x, true, comparator);
        Ok(())
    }
    else
    {
        Err(format!("The length of x is not a power of two. (x.len(): {}", x.len()))
    }
}


pub fn sort<T: Ord>(
    x: &mut [T],
    order: &SortOrder) -> Result<(), String>
{
    match *order{
        SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    }
}


fn do_sort<T, F>(
    x: &mut[T],
    forward: bool,
    comparator: &F) where F: Fn(&T, &T) -> Ordering
{
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true, comparator);
        do_sort(&mut x[mid_point..], false, comparator);
        sub_sort(x, forward, comparator);
    }
}


fn sub_sort<T, F>(
    x: &mut[T],
    forward: bool,
    comparator: &F) where F: Fn(&T, &T) -> Ordering
{
    if x.len() > 1
    {
        compare_and_swap(x, forward, comparator);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], forward, comparator);
        sub_sort(&mut x[mid_point..], forward, comparator);
    }
}


fn compare_and_swap<T, F>(
    x: &mut[T],
    forward: bool,
    comparator: &F) where F: Fn(&T, &T) -> Ordering
{
    // 比較に先立ち forward( bool値 )を Ordering値に変換しておく
    let swap_condition = if forward {
        Ordering::Greater
    }
    else
    {
        Ordering::Less
    };

    let mid_point = x.len() / 2;
    for i in 0..mid_point
    {
        // comparator クロージャで 2 要素を比較し、返された Ordering のバリアントが swap_condition と等しいなら要素を交換する

        if comparator(&x[i], &x[mid_point + i]) == swap_condition
        {
            x.swap(i, mid_point + i);
        }
    }
}


#[cfg(test)]
mod tests{

    // 親モジュール (first) のsort関数を使用する
    use super::{sort, sort_by};
    use crate::SortOrder::*;

    // derive アトリビュートを使い、Debug トレイトと PartialEq トレイトの実装を自動導出する
    #[derive(Debug, PartialEq)]
    struct Student
    {
       first_name: String,
       last_name: String,
       age: u8, // 8ビット符号なし整数
    }

    // implブロックを使うと対象の型に関連関数やメソッドを実装できる
    impl Student
    {

        fn new(
            first_name: &str,
            last_name: &str,
            age: u8
        ) -> Self {
            // 構造体 Student を初期化して返す、Self は impl 対象の型( Student )の別名
            Self{
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age, // フィールドと変数が同じ名前の時はこのように省略して記述することが出来る
            }
        }
    }


    // 氏名順にソート
    #[test]
    fn sort_students_by_name_ascending()
    {
        // 四人分のテストデータを作成
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert_eq!(sort_by(&mut x,
            // まず last_name を比較する
            &|a, b| a.last_name.cmp(&b.last_name)
                // last_name が等しくないならばそれを返す
                // last_name が等しければ first_name を比較する
                .then_with(||a.first_name.cmp(&b.first_name))), Ok(())
        );

        assert_eq!(x, expected);
    }


    // 年齢で小順にソートする
    #[test]
    fn sort_students_by_age_ascending()
    {
        // 四人分のテストデータを作成
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        // ソート対象のベクタを作成
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        // ソート後の期待値を作成
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];
        assert_eq!(
            // sort_by関数でソートする。第2引数はソート順を決めるクロージャ
            // 引数に2つのStudent構造体をとり、ageフィールドの値をcmpメソッドで
            // 比較することで大小を決定する
            sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
            Ok(())
        );

        // 結果を検証する
        assert_eq!(x, expected);
    }


    #[test]
    fn sort_to_fail()
    {
        let mut x = vec![10, 30, 11]; // x.len()が2のべき乗になっていない
        assert!(sort(&mut x, &Ascending).is_err()); // 返り値はErr
    }


    // #[test]のついた関数は cargo test としたときに実行される
    #[test]
    fn sort_u32_ascending()
    {
        // テストデータとしてu32型のベクタを作成し x に束縛する
        // sort関数によって内容が更新されるので、可変を表す mut キーワードが必要
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        // x のスライスを作成し、sort関数を呼び出す
        // &mut x は &mut x[..] と書いても良い
        assert_eq!(sort(&mut x, &Ascending), Ok(()));

        // x の要素が昇順にソートされていることを確認する
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }


    #[test]
    fn sort_u32_descending()
    {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Descending), Ok(()));

        // x の要素が降順にソートされていることを確認する
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }


    #[test]
    fn sort_str_ascending()
    {
        // 文字列のベクタを作成し、ソート
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }


    #[test]
    fn sort_str_descending()
    {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
    }
}


