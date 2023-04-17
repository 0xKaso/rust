fn main() {
    match std::env::home_dir() {
        Some(data) => println!("optins is some data = {:?}", data),
        None => println!("optins is not some data"),
    }

    match std::env::var("LANG2") {
        Ok(data) => println!("optins is some data = {:?}", data),
        Err(err) => println!("err {}", err),
    }
}
