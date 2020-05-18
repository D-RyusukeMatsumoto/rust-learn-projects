use rand::{Rng, SeedableRng};
use rand::distributions::standard;
use rand_pcg::Pcg64Mcg;
use std::mem::min_align_of_val;

pub fn new_u32_vec(
    n: usize ) -> Vec<u32>
{
    // RNG を初期化、再現性を持たせるために毎回同じシード値を利用する
    let mut rng = Pcg64Mcg::from_seed([0;16]);

    // rng.sample_iter()は乱数を無限に生成するイテレータを返す
    // take(n)は元のイテレータから最初の n 要素だけを取り出すイテレータを返す
    // collect() はイテレータから値を収集して、ベクタやハッシュマップのようなコレクションに格納する
    rng.sample_iter(&standard).take(n).collect()
}


