

pub fn sample()
{
    sample_11();
}


#[allow(dead_code)]
fn sample_0()
{
    println!("ch05_05_string1::sample_0");

    // strリテラルからStringを作る,どちらの方法でも結果は同じ
    let mut s1 = "ラズベリー".to_string();
    let mut s2 = String::from("ブラックベリー");

    // Rust 1.19より以前のバージョンでは性能上の理由からto_string()よりもto_owned()が推奨された
    // 1.19以降のバージョンではその配慮は必要ない
    let s3 = "ストロベリー".to_owned();

    s1.push_str("タルト"); // String型の文字列に &str型の文字列を追加
    assert_eq!(s1, "ラズベリータルト");

    s2.push('と'); // Stringにcharを追加する

    // push_str()が受け付けるのは&str型のみ,以下はコンパイルエラーになる
    //s2.push_str(s3); // s3はString型

    // &をつけると型強制という仕組みによって &Stringから&strへ変換される
    s2.push_str(&s3);
    assert_eq!(s2, "ブラックベリーとストロベリー");
}


// 数値型から文字列への変換及びフォーマットサンプル
fn sample_1()
{
    println!("ch05_05_string1::sample_1");

    let i = 42; // i32型
    assert_eq!(i.to_string(), "42");

    let f = 4.3 + 0.1; // f64型
    assert_eq!(f.to_string(), "4.3999999999999995");
    assert_eq!(format!("{:.2}", f), "4.40"); // format!マクロが便利

    let t = (1, "ABC");
    // 2要素のタプル型はDebugトレイトを実装しているのでformat!マクロで変換できる
    assert_eq!(format!("{:?}", t), r#"(1, "ABC")"#);
}


fn sample_2()
{
    println!("ch05_05_string1::sample_2");

    let s1 = "42";
    assert_eq!(s1.parse::<i32>(), Ok(42)); // &str型からi32型へ変換

    let s2 = "abc";
    let r2: Result<f64, _> = s2.parse(); // 変数の型から型推論できるならparseの型パラメータは不要
    assert!(r2.is_err()); // 数値として解釈できないときはエラーが帰る
    println!("{:?}", r2); // -> Err(ParseFloatError { kind: Invalid})
}


// charやバイト列からStringへ
fn sample_3()
{
    println!("ch05_05_string1::sample_3");

    let cs = ['t', 'r', 'u', 's', 't']; // [char; 5]型
    assert_eq!(cs.iter().collect::<String>(), "trust");
    assert_eq!(&cs[..1].iter().collect::<String>(), "t");
    assert_eq!(&cs[1..].iter().collect::<String>(), "rust");
}


// UTF-8として不正な値を含むバイト列からでもStringを作成できる
fn sample_4()
{
    let bad_utf8: [u8; 7] = [
        b'a',             // a
        0xf0, 0x90, 0x80, // でたらめなバイト列
        0xe3, 0x81, 0x82  // あ
    ];

    // 不正なバイト列は UnicodeのU+FFFD Replacement Characterに置き換わる
    let s = String::from_utf8_lossy(&bad_utf8);
    assert_eq!(s, "a\u{fffd}あ");
}


// &strは参照先のUTF-8バイト列よりも生存期間が短い,そのため関数から返す時は注意が必要
// 以下のコードはコンパイルエラーになるサンプル
fn sample_5()
{
    //f1(&"Matsumoto");
    f2("Matsumoto");
}

// こちらはコンパイルエラーが発生する.
/*
fn f1(name: &str) -> &str
{
    let s = format!("Hello, {}!", name); // format!はStringを作成する
    &s // Stringから&strを作成し,返り値として返す
    // -> コンパイルエラー: 's" does not live long enough. ( s の生存期間が不十分 )
}
*/

// こちらがコンパイルエラーの発生しない挙動
fn f2(name: &str) -> String
{
    format!("Hello, {}", name)
}


// u16で作成した文字列をStringへ変換するサンプル
fn sample_6()
{
    let utf16: Vec<u16> = vec![0x61, 0x62, 0x6f22, 0x5b57];

    // Vec<u16>の値をUTF-16と解釈し,Stringを悪性する( UTF-8へ変換される )
    if let Ok(s) = String::from_utf16(&utf16)
    {
        assert_eq!(s, "ab漢字");
    }
    else
    {
        unreachable!();
    }
}


// バイト文字列リテラルでは文字列からu8の配列( &'static[u8; n] が作成できる
fn sample_7()
{
    // バイト文字列リテラル,ASCII文字以外のバイトは [\x2桁の16進数] で記述する
    let bs1 = b"abc\xe3\x81\x82"; // &[u8; 6]型,UTF-8表現で "abcあ"
    assert_eq!(bs1, &[b'a', b'b', b'c', 0xe3, 0x81, 0x82]);

    // rawバイト文字列リテラル,エスケープ文字(\)を特別扱いしないので \n は改行文字ではなく文字通り \n と解釈される
    let bs2 = br#"ab\ncd"#; // &[u8, 6]型
    assert_eq!(bs2, &[b'a', b'b', b'\\', b'n', b'c', b'd']);
}


// 範囲
// start..end, start..=end, start.., ..end 等の形をとり,数列の作成やスライスの範囲指定などで使われる
// ユーザ定義型として実現される.
fn sample_8()
{
    let a = ['a', 'b', 'c', 'd', 'e'];

    // 糖衣構文と実際の範囲の対応
    assert_eq!(a[.. ],   ['a', 'b', 'c', 'd', 'e']);
    assert_eq!(a[ ..3],  ['a', 'b', 'c',         ]);
    assert_eq!(a[ ..=3], ['a', 'b', 'c', 'd',    ]);
    assert_eq!(a[1.. ],  [     'b', 'c', 'd', 'e']);
    assert_eq!(a[1.. 3], [     'b', 'c',         ]);
    assert_eq!(a[1..=3], [     'b', 'c', 'd',    ]);

    // 糖衣構文とRange*型の対応
    assert_eq!( .. ,   std::ops::RangeFull );
    assert_eq!( .. 3,  std::ops::RangeTo{ end: 3 });
    assert_eq!( ..=3,  std::ops::RangeToInclusive{ end: 3});
    assert_eq!( 1.. ,  std::ops::RangeFrom{ start: 1} );
    assert_eq!( 1.. 3, std::ops::Range{ start: 1, end: 3});
    assert_eq!( 1..=3, std::ops::RangeInclusive::new(1, 3));
}


// Option
fn sample_9()
{
    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a')); // インデックス0は配列 a1 の範囲内なので Some(&値) が返る
    assert_eq!(a1.get(4), None);       // インデックス4は範囲外なので None が返る

    let mut o1 = Some(10);                 // Option<i32>型
    match o1 {                             // match式でバリアントが判別できる
        Some(s) => assert_eq!(s, 10), // パターンマッチで中の値を取り出す
        None => unreachable!(),
    }

    o1 = Some(20);
    if let Some(s) = o1 { // if let 式でもバリアントの判別と値の取り出しができる
        assert_eq!(s, 20);
    }

    // Option型にはアンラップに便利なメソッドがいくつか定義されている
    let mut o2 = Some(String::from("Hello")); // Option<String>型
    assert_eq!(o2.unwrap(), "Hello");            // unwrap()でSomeの中の値が取り出せる

    // しかしunwrap()はNoneの時にpanicするので,出来るだけ使わないほうがいい
    o2 = None;
    //o2.unwrap(); // <- このシチュエーションの時にpanicになる

    // unwrap_or_else()ならNoneでもpanicしないので安心して利用出来る
    // Noneの時はクロージャを実行し,Noneの代わりになる値を得る
    assert_eq!(o2.unwrap_or_else(|| String::from("o2 is none")), "o2 is none");

    // Someで包まれた値を操作するならmap()やand_then()等のコンビねーたが便利
    // map()はSome(値)の時は値にクロージャを適用し,クロージャが返した値をSomeで包みなおす
    let mut o3 = Some(25);
    assert_eq!(o3.map(|n| n * 10), Some(250));

    // Noneなら何もせずNoneを返す
    o3 = None;
    assert_eq!(o3.map(|n| n * 10), None);

    o3 = Some(10);
    assert_eq!(
        o3.map(|n| n * 10)
            // and_then()はSome(値)の時は値にクロージャを適用し
            // クロージャが返した値を( Some(新しい値) または None )をそのまま返す
            .and_then(|n| if n >= 200 { Some(n) } else { None }),
            None);
}


fn sample_10()
{
    // インデックス0と3の両方に値があるのでそれらの合計がSomeで包まれて返される
    assert_eq!(add_elems(&[3, 7, 31, 127]), Some(3 + 127));

    // インデックス3がないのでNoneが返される
    assert_eq!(add_elems(&[7, 11]), None);
}


// 複数のオプション値がすべてSomeの時に処理を実行したいなら,?演算子が便利
fn add_elems(s: &[i32]) -> Option<i32> {

    // 複数のOption値を扱うときは?演算子が便利
    // Some( 値 )なら値を取り出し,Noneならこの関数からすぐに戻る( Noneを返す )
    let s0 = s.get(0)?;
    let s3 = s.get(3)?;
    Some(s0 + s3)
}


// Result<T, E>のサンプル
// Result型の返り値は結果がエラーになる可能性を暗示する型
fn sample_11()
{
    // str::parse()は文字列をしていした型( ここではi32型 )に変換する
    assert_eq!("10".parse::<i32>(), Ok(10)); // 変換出来たらOk(10)が返される
    let res0 = "a".parse::<i32>(); // 変換できなかったら Err(エラーを表す値) が返される
    assert!(res0.is_err());
    println!("{:?}", res0); // -> Err(ParseIntError { kind: InvalidDigit })
}


// リザルト型の返り値を持つ関数を利用する例
// Errorが出たときのサンプルも併記
fn sample_12()
{
    assert_eq!(add0("3", "127"), Ok(3 + 127));
    assert!(add0("3", "abc").is_err());
}


// オプション値と同様に複数のリザルト値を扱うときは?演算子が便利
// Ok(値)なら値を取り出し,Err(エラーを表す値)ならこの関数からリターンする
fn add0(
    s0: &str,
    s1: &str)
    -> Result<i32, std::num::ParseIntError>
{
    let s0 = s0.parse::<i32>()?;
    let s1 = s1.parse::<i32>()?;
    Ok(s0 + s1)
}


// map_err()コンビネータを利用してのエラー書き換えの例
fn sample_13()
{
    assert_eq!(add1("3", "abc"), Err("s1が整数ではありません".to_string()));
}


// map_err()コンビネータを利用してのエラー書き換えの例
fn add1(
    s0: &str,
    s1: &str)
    -> Result<i32, String>
{
    let s0 = s0.parse::<i32>().map_err(|_e| "s0が整数ではありません")?;
    let s1 = s1.parse::<i32>().map_err(|_e| "s1が整数ではありません")?;
    Ok(s0 + s1)
}


// 型エイリアス
type UserName = String;
type Id = i64;
type Timestamp = i64;
type User = (Id, UserName, Timestamp);


// 型エイリアスで定義した別名のものを生成するサンプル
fn sample_14()
{
    let id = 400;
    let now = 4567890123;
    let user = new_user(String::from("mika"), id, now);

    // IdとTimestampは同じi64型なので間違えてもエラーにならない
    let bad_user = new_user(String::from("kazuki"), now, id);

}


// 型エイリアスで定義した別名のものを生成するサンプル
fn new_user(
    name: UserName,
    id: Id,
    created: Timestamp)
    -> User
{
    (id, name, created)
}


// 構造体

// 名前付きフィールド構造体
// 構造体のフィールドでは型指定の省略はできない
#[derive(Default)] // Defaultトレイトを実装することでフィールドのデフォルト値が設定できる
struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}


// ユニット構造体
struct UniqueValue;
// または
// struct UniqueValue {}
// struct UniqueValue();


fn sample_15() {

    // Polygon型の値を作り,変数triangleを束縛する
    let triangle = Polygon{
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 1,
    };

    // 1.17よりフィールド初期化略記構文が導入され以下のような初期化も可能
    let quadrangle = new_polygon(vec![(5, 2), (4, 7), (10, 6), (8, 1)]);

    // フィールドへのアクセスは2通りある
    // フィールド名を利用する方法, パターンマッチで分解する方法

    // フィールド名でアクセスする方法
    assert_eq!(triangle.vertexes[0], (0, 0));
    assert_eq!(triangle.vertexes.len(), 3);
    assert_eq!(triangle.fill, (255, 255, 255));

    // パターンマッチでのアクセス
    // 不要なフィールドは .. で省略できる
    let Polygon { vertexes: quad_vx, .. } = quadrangle;
    assert_eq!(4, quad_vx.len());

    // :以降を省略するとフィールドと同じ名前の変数が作られ,フィールド値に束縛される
    let Polygon { fill, .. } = quadrangle;
    assert_eq!((0, 0, 0), fill);

    // 構造体の値を変更するにはmutが必要
    let mut polygon = new_polygon(vec![(-1, -5), (-4, 0)]);
    assert_eq!(polygon.vertexes.len(), 2);
    polygon.vertexes.push((2, 8));
    assert_eq!(polygon.vertexes.len(), 3);

    // 既存の値をもとにしてその一部を利用した新しい値を作成することが出来る
    let triangle1 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 5
    };

    // triangle1を元にvertexesだけ異なる新しい値を作成する
    let triangle2 = Polygon {
        vertexes: vec![(0, 0), (-3, 0), (2, 2)],
        .. triangle1
    };
    // この構文は関数型レコードアップデート構文と呼ぶ

    // #[derive(Default)]を実装している構造体であれば以下のように
    // すべてのフィールドがデフォルト値を持つPolygonを作成する
    let polygon1: Polygon = Default::default();
}


fn new_polygon(
    vertexes: Vec<(i32, i32)>) -> Polygon {

    let stroke_width = 1;
    let fill = (0, 0, 0);
    Polygon{ vertexes, stroke_width, fill }
}


// タプル構造体
// タプルのような構造体,フィールドに名前を与えず,0から始まる連番のフィールド名を用いる
struct Vertex (i32, i32);
struct Triangle (Vertex, Vertex, Vertex);


fn sample_16() {

    let vx0 = Vertex(0, 0);
    let vx1 = Vertex(3, 0);
    let triangle = Triangle(vx0, vx1, Vertex(2, 2));
    assert_eq!((triangle.1).0, 3);

}





#[cfg(test)]
mod test
{
    /*
    use super::sample_0;
    use super::sample_1;
    use super::sample_2;
    use super::sample_3;
    use super::sample_4;
    use super::sample_5;
    use super::sample_6;
    use super::sample_7;
    use super::sample_8;
    use super::sample_9;
    use super::sample_10;
    use super::sample_11;
    use super::sample_12;
    use super::sample_13;
    use super::sample_14;
    use super::sample_15;
    use super::sample_16;
    */

    /*
    #[test] fn test_0() { sample_0(); }
    #[test] fn test_1() { sample_1(); }
    #[test] fn test_2() { sample_2() }
    #[test] fn test_3() { sample_3(); }
    #[test] fn test_4() { sample_4(); }
    #[test] fn test_5() { sample_5(); }
    #[test] fn test_6() { sample_6(); }
    #[test] fn test_7() { sample_7(); }
    #[test] fn test_8() { sample_8(); }
    #[test] fn test_9() { sample_9(); }
    #[test] fn test_10() { sample_10(); }
    #[test] fn test_11() { sample_11(); }
    #[test] fn test_12() { sample_12(); }
    #[test] fn test_13() { sample_13(); }
    #[test] fn test_14() { sample_14(); }
    #[test] fn test_15() { sample_15(); }
    #[test] fn test_16() { sample_16(); }
    */

}








