fn main() {
    convert_c2f(25);
    convert_f2c(77);
}

// 摂氏 -> 華氏を変換する関数
fn convert_c2f(n: i32) -> () {
    let answer = (n * 9 / 5) + 32;
    println!("answer: {}", answer);
}

// 摂氏 <- 華氏を変換する関数
fn convert_f2c(n: i32) -> () {
    let answer = (n - 32) * 5 / 9;
    println!("answer: {}", answer);
}
