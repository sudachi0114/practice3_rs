fn main() {
    let n = 7;
    let answer = fibonacci(n);
    println!("The {}th item of fibonacci-sq is {}", n, answer);

    let mut idx = 1;
    while idx < 15 {
        let answer = fibonacci(idx);
        println!("The {}th item of fibonacci-sq is {}", idx, answer);

        idx = idx + 1;
    }
}

// フィボナッチ数列の第n項目を求めるプログラム
fn fibonacci(n: u32) -> u32 {
    let answer = if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    };
    answer
}
