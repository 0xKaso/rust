fn lagest(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

fn lagestFloat(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

// 泛型，类型不确定，T Type
fn lagestRun<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if (a > b) {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", lagest(11, 23));
    println!("{}", lagestFloat(11.32132, 23.11));

    println!("{}", lagestRun(1, 232));
    println!("{}", lagestRun(1.1, 2.32));
}
