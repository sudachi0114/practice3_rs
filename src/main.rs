fn main() {
    convert_c2f(10);
}

// 摂氏 -> 華氏を変換する関数
fn convert_c2f(n: i32) -> () {
    let answer = (n * 9 / 5) + 32;
    println!("answer: {}", answer);
}
