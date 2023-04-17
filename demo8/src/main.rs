fn main() {
    // if 表达式
    if 10 > 3 {
        println!("Yes!");
    } else {
        println!("No!");
    }

    if true {
        println!("true!");
    }


    if true { 1 } else { "1" } // ❎ 此处由于无法确定返回值类型，故写法错误
}
