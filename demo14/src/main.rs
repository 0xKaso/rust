fn append_string(s: &mut String) {
    s.push_str("!");
}

fn main() {
    let mut str = String::from("Hi");
    append_string(&mut str);
    println!("{}", str);
}
