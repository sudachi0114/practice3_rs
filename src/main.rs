fn main() {
    convert_c2f(25);
    convert_f2c(77);

    convert_cf(25, 'c');
    convert_cf(77, 'f');
    convert_cf(9999, 'z');
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

// 摂氏 <-> 華氏を変換する関数
//   n   : 数値 (摂氏や華氏で表された温度の数値)
//   unit: 単位 (摂氏: c / 華氏: f)
fn convert_cf(n: i32, unit: char) -> () {
    let answer = if unit == 'c' {  // 摂氏 -> 華氏
        (n * 9 / 5) + 32
    } else if unit == 'f' {  // 華氏 -> 摂氏
        (n - 32) * 5 / 9
    } else {  // unit が 'c' でも 'f' でもない場合
        -1  // raise Error
    };

    let answer_unit = if unit == 'c' {
        'f'
    } else if unit == 'f' {
        'c'
    } else {
        'x' // raise Error
    };

    if answer != -1 || answer_unit != 'x' {
        println!("answer: {} {}", answer, answer_unit);
    } else {
        println!("[Error] something wrong... @input_parameter: ({}, {})", n, unit);
    }
}
