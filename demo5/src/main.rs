// 1 1
// 1 2 1
fn fib(n: u32) {
    let mut a = 1;
    let mut b = 1;
    let mut max = 2u32;

    loop {
        println!("next value is :{}", b);
        let c = a + b;
        a = b;
        b = c;
        
        max += 1;
        if max > n {
            break;
        }
    }
}

fn main() {
    fib(10000);
}
