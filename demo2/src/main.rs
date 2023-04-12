// 函数需注明参数类型和返回值类型
fn add(value1: i32, value2: i32) -> i32 {
    value1 + value2 // 此处不加;为返回值
}

fn div(value1: i32, value2: i32) -> i32 {
    value1 / value2 // 此处不加;为返回值
}

fn main() {
    println!("9+3={}", add(9, 3));
    println!("9/3={}", div(9, 3));
}
