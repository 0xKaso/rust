use std::rc::Rc; // 处理单个值拥有多个所有者的情况，当所有者个数为0，则清除变量

struct Node<T> {
    value: T,
    left: Option<Rc<Node<T>>>,
    right: Option<Rc<Node<T>>>,
}

fn main() {
    let mut root = Node::<u32> {
        value: 0,
        left: None,
        right: None,
    };
    let left = Node::<u32> {
        value: 1,
        left: None,
        right: None,
    };
    let right = Node::<u32> {
        value: 2,
        left: None,
        right: None,
    };

    root.left = Some(Rc::new(left));
    root.right = Some(Rc::new(right));

    // println!("root = {:?}", root.value);
    // println!("left = {:?}", root.left.unwrap().value);
    // println!("right = {:?}", root.right.unwrap().value);

//     if let Some(ref mut x)  = root.left { 
//         x.borrow_mut().value = 4;
//     }

//     if let Some(ref x) = root.left {
//         println!("left = {:?}", x.borrow().value);
//     }
}
