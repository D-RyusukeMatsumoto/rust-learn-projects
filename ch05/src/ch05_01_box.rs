
#[allow(dead_code)]
pub fn sample(){
    println!("ch05_01_box.rs");
    sample_0();
}


#[allow(dead_code)]
fn sample_0(){
    println!("ch5 sample_0");

    let t1 = (3, "birds".to_string()); // (i32, String)型のタプル,スタックに置かれる
    let mut b1 = Box::new(t1);       // Boxポインタを作る,タプルがヒープに移動する
    (*b1).0 += 1;                                // *で参照外し
    assert_eq!(*b1, (4, "birds".to_string()));
}


#[cfg(test)]
mod test {

    use super::sample_0;
    use crate::r#box::sample;

    #[test]
    fn test_0(){
        sample_0();
    }



}








