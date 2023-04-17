
const A:i32 = 100; // 常量，需要指明类型

fn main(){
    // 整数
    // u8, u16, u32, u64, usize
    // i8, i16, i32, i64, isize 
    // i32 最快
    let mut x = 10;
    let y = -10;

    // 浮点数
    // f32, f64
    let pi = 3.1415926;

    // 字符
    let emoj = '😄';

    // 数组
    let persons:[i32;5] = [1,2,3,4,5];

    x = 10;

    println!("{}", x);
    println!("{}", y);
    println!("{}", pi);
    println!("{}", A);
    println!("{}", emoj);
    println!("{}", persons[1]);
}

// SSA 一个变量只能使用一次，用完即丢