use rand::{Rng, SeedableRng};
use rand::distributions::standard;
use rand_pcg::Pcg64Mcg;
use std::mem::min_align_of_val;

pub fn new_u32_vec(
    n: usize ) -> Vec<u32>
{
    // RNG を初期化、再現性を持たせるために毎回同じシード値を利用する
    let mut rng = Pcg64Mcg::from_seed([0;16]);

    // n個の要素が格納できるようベクタを初期化する
    let mut v = Vec::with_capacity(n);

    // 0 から n - 1 までの合計 n回、繰り返し乱数を生成し、ベクタに追加する
    // (0 から n - 1 の数列は使わないので _ で受けることで、すくに破棄している)
    for _ in 0..n
    {
        // RNG の sample メソッドは引数として与えられた分布に従う乱数を一つ生成する
        // Standard 分布は生成する値が数値型 ( ここでは u32型 ) の時は一様分布になる、その方がとりうるすべての値が同じ確率で出現する
        v.push(rng.sample(&standard));
    }

    // 生成したベクタを返す
    v
}