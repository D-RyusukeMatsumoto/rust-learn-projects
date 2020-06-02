

// 文字列スライス
pub fn sample() {
    println!("fourth.rs");

    let s1 = "abc1"; // &'static str型
    let s2 = "abc2";
    assert!(s1 < s2);
    assert!(s1 != s2);

    let s3 = "文字列を複数行にわたって書くと
        改行やスペースが入る";
    println!("{}", s3);

    let s4 = "行末にバックスラッシュを付けると\
        改行などが入らない";
    println!("{}", s4);
    assert_eq!(s3, "文字列を複数行にわたって書くと\n        改行やスペースが入る");
    assert_eq!(s4, "行末にバックスラッシュを付けると改行などが入らない");

    let s5 = "文字列に\"と\\を含める"; // バックスラッシュでエスケープ
    println!("{}", s5);
    let s6 = r#"文字列に"と\を含める"#; // raw文字列リテラル,正規表現などに便利
    println!("{}", s6);
    assert_eq!(s5, s6);

    let s7 = r###"このように#の数を増やすと"##"があっても大丈夫"###;
    println!("{}", s7);
    assert_eq!(s7, "このように#の数を増やすと\"##\"があっても大丈夫");

    // コンソールでは利用できない.
    //let s8 = "もちろん絵文字\u{1f600}も使える"; // もちろん絵文字絵文字も使える

    let fruits = "あかりんご, あおりんご\nラズベリー, ブラックベリー";

    // lines() 関数は改行コードを含むコード(\n)を含む文字列から1行ずつ取り出せるイテレータを作る
    let mut lines = fruits.lines();

    // イテレータの next() で次の行を得る
    let apple_line = lines.next();
    assert_eq!(apple_line, Some("あかりんご, あおりんご"));
    assert_eq!(lines.next(), Some("ラズベリー, ブラックベリー"));

    // 次の行がないなら None を返す
    assert_eq!(lines.next(), None);

    // リンゴの行 (Some(..)) の中身を取り出す
    if let Some(apples) = apple_line {
        // [あか]で始まるかチェック
        assert!(apples.starts_with("あか"));

        // [りんご]の文字列を含むかチェック
        assert!(apples.contains("りんご"));

        // [あお]が最初に出現する位置( UTF-8 表現で何バイト目 )を得る
        assert_eq!(apples.find("あお"), Some(17)); // 0始まりなので18バイト目

        // 文字列を , で分割するイテレータを作る
        let mut apple_iter = apples.split(",");
        assert_eq!(apple_iter.next(), Some("あかりんご"));

        let green = apple_iter.next();

        // 左側に余白がある
        assert_eq!(green, Some("あおりんご"));

        // Some(..)の内容にstrのtrim()を適用して余白を取り除く
        assert_eq!(green.map(str::trim), Some("あおりんご"));

        assert_eq!(apple_iter.next(), None);
    } else {
        unreachable!(); // もしここに到達したらパニックで強制終了する
    }




}


