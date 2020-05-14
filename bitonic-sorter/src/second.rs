// pub はこのsort関数がほかのモジュールからアクセスできることを示す
// 引数 x の型　&mut[u32]について
// & は値をポインタ経由で借用することを示す
// [u32]型は u32 のスライス ( 現時点でスライスは1次元の配列と考えてよい )
pub fn sort<T>(x: &mut[T], up: bool){
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}


fn sub_sort<T>(x: &mut[T], up: bool){
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}


fn compare_and_swap<T>(x: &mut[T], up: bool){
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (x[i] > x[mid_point + i]) == up {
            // 要素を交換する
            x.swap(i, mid_point + i);
        }
    }
}


#[cfg(test)]
mod tests{

    // 親モジュール (first) のsort関数を使用する
    use super::sort;

    // #[test]のついた関数は cargo test としたときに実行される
    #[test]
    fn sort_u32_ascending(){
        // テストデータとしてu32型のベクタを作成し x に束縛する
        // sort関数によって内容が更新されるので、可変を表す mut キーワードが必要
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        // x のスライスを作成し、sort関数を呼び出す
        // &mut x は &mut x[..] と書いても良い
        sort(&mut x, true);

        // x の要素が昇順にソートされていることを確認する
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }


    #[test]
    fn sort_u32_descending(){
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, false);

        // x の要素が降順にソートされていることを確認する
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }
    
    
    #[test]
    fn sort_str_ascending(){
        // 文字列のベクタを作成し、ソート
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        sort(&mut x, true);
        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }
    
    
    #[test]
    fn sort_str_descending(){
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        sort(&mut x, false);
        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
    }
}


