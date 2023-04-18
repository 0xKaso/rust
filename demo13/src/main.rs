// 引用与借用
// 可以理解成传值和传引用

//

fn reverse_string2(s: String) -> String {
    s.chars().rev().collect()
}

fn reverse_string(s: &String) -> String {
    s.chars().rev().collect()
}

fn main() {
    let s = String::from("hello world");

    println!("{}", reverse_string(&s)); // 引用：不拥有字符串，所指向的值不会被函数删除(该值可以再次被访问)
    println!("{}", reverse_string(&s));
    println!("{}", reverse_string(&s));
    println!("{}", reverse_string(&s));

    println!("{}", reverse_string2(s)); // 借用：拥有字符串，所指向的值最终会被函数删除(该值不可以再次被访问)
}
