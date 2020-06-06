
#[allow(dead_code)]
pub fn sample()
{
    sample_1();
    sample_2();
    sample_3();
    sample_4();
}


// strの長さ
#[allow(dead_code)]
fn sample_1()
{
    println!("fifth : sample_1");
    // s1 から s2 どちらも画面上では一文字として表示される
    // UTF-8表現
    let s1 = "a";  // 61
    let s2 = "あ"; // E3 81 82

    assert_eq!(s1.len(), 1);
    assert_eq!(s2.len(), 3);
}


#[allow(dead_code)]
fn sample_2()
{
    let s = "abcあいう";
    assert_eq!(s.get(0..1), Some("a"));
    assert_eq!(s.get(3..6), Some("あ"));
    assert_eq!(s.get(3..4), None); // UTF-8として解釈できない場合
}


// strと他の型の変換
#[allow(dead_code)]
fn sample_3()
{
    let s = "かか\u{3099}く"; // \u{3099}は濁点文字
    println!("{}", s);            // かがく

    // char型のイテレータを取得し,それぞれの文字を比較
    let mut iter = s.chars();
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), Some('\u{3099}'));
    assert_eq!(iter.next(), Some('く'));
    assert_eq!(iter.next(), None);
}


#[allow(dead_code)]
fn sample_4()
{
    let utf8: [u8; 4] = [0x61, 0xe3, 0x81, 0x82];
    assert_eq!(std::str::from_utf8(&utf8), Ok("aあ"));

    let bad_utf8: [u8; 2] = [0x81, 0x33]; // でたらめなバイト列
    let result2 = std::str::from_utf8(&bad_utf8);
    assert!(result2.is_err());
    println!("{:?}", result2);
    // "Err( Utf8Error{ valid_up_to: 0, error_len: Some(1)} )"

}


// 可変のstr
#[allow(dead_code)]
fn sample_5() {
    // 文字リテラル( &'static str )から&mut strは直接得られない
    // まず文字列リテラルをStringへ変換し,そこから&mut strを取り出す
    let mut s1 = "abcあいう".to_string();

    // &mut strを得る,これはStringが持つUTF-8バイト列を指す可変スライス
    let s2 = s1.as_mut_str(); // &mut str型

    // 英小文字を大文字に変換
    s2.make_ascii_uppercase();
    assert_eq!(s2, "ABCあいう");

    // &mut strのUTF-8バイト列を直接操作して"あ"( 3バイト )を"*a*"に変更する
    let b = unsafe{ s2.as_bytes_mut() };
    b[3] = b'*';
    b[4] = b'a';
    b[5] = b'*';

    // 大元のStringが変更されている
    assert_eq!(s1, "ABC*a*いう");
}


#[cfg(test)]
mod tests
{

    use super::sample_1;
    use super::sample_2;
    use super::sample_3;
    use super::sample_4;
    use super::sample_5;


    #[test]
    fn test_1()
    {
        sample_1();
    }


    #[test]
    fn test_2()
    {
        sample_2();
    }


    #[test]
    fn test_3()
    {
        sample_3();
    }

    #[test]
    fn test_4()
    {
        sample_4();
    }

    #[test]
    fn test_5(){ sample_5(); }
}





