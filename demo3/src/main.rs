fn pi() -> f64 {
    3.14
}

fn not_pi() {
    3.14;
}

fn main() {
    println!("pi:{}", pi());
    // println!("not pi:{}", not_pi()) 由于这里没有返回值，所以这样写会报错
}
