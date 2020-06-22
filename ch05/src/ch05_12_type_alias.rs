

struct UserName(String);
struct Id(u64);
struct Timestamp(u64);
type User = (Id, UserName, Timestamp);

// 新規ユーザの作成
fn new_user(
    name: UserName,
    id: Id,
    created: Timestamp) -> User {
    (id, name, created)
}






fn sample() {
    println!("ch05_12_type_alias.rs");

    let id = Id(400);
    let now = Timestamp(4567890123);

    // now と id の順番を間違えるとコンパイルエラーになるので順番の間違えによるバグが防げる
    //let bad_user = new_user(UserName(String::from("kazuki")), now, id);
    // error[E0308]: mismatched types
    // expected type 'id', found type 'Timestamp'
    let bad_user = new_user(UserName(String::from("kazuki")), id, now);
}



#[cfg(test)]
mod test {

    use super::sample;

    #[test]
    fn test_0() { sample(); }



}

