
// 各サンプルの名前をこのファイルで利用する名前に変更
#[allow(unused_imports)]
use ch04_15_slice1::first::sample as first_sample;

#[allow(unused_imports)]
use ch04_15_slice1::second::sample as second_sample;

#[allow(unused_imports)]
use ch04_15_slice1::third::sample as third_sample;

#[allow(unused_imports)]
use ch04_15_slice1::fourth::sample as fourth_sample;

#[allow(unused_imports)]
use ch04_15_slice1::fifth::sample as fifth_sample;


fn main() {
    first_sample();
    second_sample();
    third_sample();
    fourth_sample();
    fifth_sample();
}


#[cfg(tests)]
mod tests {

    use super::first_sample;
    use super::second_sample;
    use super::third_sample;
    use super::fourth_sample;
    use super::fifth_sample;

    #[test]
    fn first_test() {
        first_sample();
    }


    #[test]
    fn second_test() {
        second_sample();
    }


    #[test]
    fn third_test() {
        third_sample();
    }


    #[test]
    fn fourth_test() {
        ourth_sample();
    }


    #[test]
    fn fifth_test() {
        fifth_sample();
    }
}


