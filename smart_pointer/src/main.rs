use std::ops::Deref;

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Drop CustomSmartPointer: {}", self.data);
    }
}

fn main() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(
            2,
            Box::new(List::Cons(
                3,
                Box::new(
                    List::Nil
                )
            ))
        ))
    );
    
    println!("list = {:?}", list);

    let a = MyBox::new(42);    
    assert_eq!(42, *a);

    let b = MyBox::new("World");
    hello(&b);

    let _c = CustomSmartPointer { data: String::from("Hello World") };
    let d = CustomSmartPointer { data: String::from("Another Data") };
    std::mem::drop(d);
    println!("CustomSmartPointer instantiated");    
}

fn hello(msg: &str) {
    println!("Hello {}", msg);
}