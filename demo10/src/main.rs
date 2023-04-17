struct User {
    username: String,
    age: u32,
    email: String,
}

// 函数
fn sayHi(user: &User) {
    println!(
        "Hello, I'm {} and {} years old, My email is {}.",
        user.username, user.age, user.email
    );
}


// 方法
impl User {
    fn sayHi(self) {
        // self 类似 this
        println!(
            "😄 Hello, I'm {} and {} years old, My email is {}.",
            self.username, self.age, self.email
        );
    }
}

fn main() {
    let mut newage = 100;
    let mut someone = User {
        username: String::from("kaso"),
        age: 35,
        email: String::from("aaa@gmail.com"),
    };

    someone.username = String::from("Lukas");

    println!("user name is: {}", someone.username);
    sayHi(&someone); // & 表示指针
    someone.sayHi();
}
