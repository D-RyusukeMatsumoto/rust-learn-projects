


fn main() {

    //sample0();
    //sample1();
    //sample2();
    //sample3();
    sample4();

}


// 簡単な配列の作成サンプル
#[allow(dead_code)]
fn sample0() {
    println!("Sample0");

    let array1 = ['H', 'e', 'l', 'l', 'o'];
    assert_eq!(array1[1], 'e');
}


// 配列へのアクセス方法
#[allow(dead_code)]
fn sample1() {
    println!("Sample1");

    let mut array = [0, 1, 2];
    array[1] = 10;
    assert_eq!(array, [0, 10, 2]);

    // インデックスは定数でなくても構わない
    let mut index = 0;
    assert_eq!(array[index], 0);

    index += 1;
    assert_eq!(array[index], 10);
}


// 直接インデックス番号を指定しない要素へのアクセス方法
#[allow(dead_code)]
fn sample2() {
    println!("Sample2");

    let array = [0, 1];
    // let num = array3[2]; // 実行時にパニックになる

    // get()はインデックスが範囲内の時はSome(&値)を返す
    // 指定したインデックスが範囲外の場合はNoneを返す
    assert_eq!(array.get(1), Some(&1));
    assert_eq!(array.get(2), None);
}


// イテレータの使用
#[allow(dead_code)]
fn sample3() {
    println!("Sample3");

    // イテレータの使用
    let array = ['a'; 50]; // 長さ50の a 文字で埋められた配列を作成

    // iter()で要素が不変のイテレータを作成
    for ch in array.iter() {
        print!("{},", *ch);
    }
}


// 要素が可変のイテレータの使用
#[allow(dead_code)]
fn sample4() {
    println!("Sample4");

    let mut array = [1; 50]; // 長さ50の1で埋められた配列を作成

    println!("Before");

    for befor in array.iter() {
        print!("{},", befor);
    }
    print!("\n");

    println!("Changing");
    // itr_mut()で要素が可変のイテレータを作成
    for n in array.iter_mut() {
        *n *= 2;
    }

    println!("After");
    for after in array.iter() {
        print!("{},", after);
    }
}
