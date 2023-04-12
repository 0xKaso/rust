// #[derive(Debug)]
// 修饰符，为数据结构引入额外的行为
// 如Debug则为项目提供了debug能力
// Copy/Clone则让数据结构可以被复制

#[derive(Debug)]
// 枚举类型
enum Gender {
    Female,
    Male,
}

#[derive(Debug, Copy, Clone)]
// (元组)结构体，适用于简单的结构体
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
// 标准结构体
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

// 定义聊天室中可能发生的事件
#[derive(Debug)]
// 标签联合结构体
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn main() {
    let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female }; 
    let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };

    let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
    
    let event1 = Event::Join((alice.id, topic.id)); 
    let event2 = Event::Join((bob.id, topic.id)); 
    let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));

    println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);
}
