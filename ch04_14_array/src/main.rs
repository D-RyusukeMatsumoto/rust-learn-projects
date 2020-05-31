


fn main() {
    let array1 = ['H', 'e', 'l', 'l', 'o'];
    assert_eq!(array1[1], 'e');

    let mut array2 = [0, 1, 2];
    array2[1] = 10;
    assert_eq!(array2, [0, 10, 2]);

    // インデックスは定数でなくても構わない
    let mut index = 0;
    assert_eq!(array2[index], 0);

    index += 1;
    assert_eq!(array2[index], 10);

}
