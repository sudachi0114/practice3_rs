fn main() {
    let n = 7;
    let answer = fibonacci(n);
    println!("The {}th item of fibonacci-sq is {}", n, answer);

    let mut idx = 1;
    while idx < 15 {
        let answer = fibonacci(idx);
        println!("[check] The {}th item of fibonacci-sq is {}", idx, answer);

        idx = idx + 1;
    }

    let fl_ans = fibonacci_loop(n);
    println!("[fibonacci-loop] answer: {}", fl_ans);
}

// フィボナッチ数列の第n項目を求めるプログラム
fn fibonacci(n: u32) -> u32 {
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}

fn fibonacci_loop(n: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;
    if n > 2 {
        for _ in 2..n {
            let tmp = b;
            b = b + a;
            a = tmp;
        }
        b  // return
    } else {
        1
    }
}