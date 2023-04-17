fn main() {
    let mut times = 3;
    let mut runState = 0;

    // loop 循环
    loop {
        println!("Loop run! index {}", runState);
        runState += 1;

        if times <= runState {
            break; // break 是 break 出 main 函数
        }
    }

    runState = 0;
    
    // while 循环
    while runState < times { 
        println!("While run! index {}", runState);
        runState += 1;     
    }

    // for 循环
    let arr = [12,22,23,24,25];

    for element in arr.iter() {
        println!("{}", element);
    }
}
