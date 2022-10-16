use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

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

enum Cyclic {
    Loop(RefCell<Rc<Cyclic>>),
    None,
}

impl Cyclic {
    fn ret(&self) -> &RefCell<Rc<Cyclic>> {
        match *self {
            Cyclic::Loop(ref elem) => elem,
            Cyclic::None => panic!(),
        }
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

    let num = Rc::new(42);
    println!("ref counter: {}", Rc::strong_count(&num));
    // Rcではcloneするときに<variable>.clone()ではなくRc::clone(<variable>)と記述する
    // Rc::cloneを使うことで、他のcloneのようにディープコピーするのではなく
    // 参照カウンタを増加させるだけの処理であることがひと目でわかる。
    let another = Rc::clone(&num);
    println!("num = {}, another = {}", num, another);
    println!("ref counter: {}", Rc::strong_count(&num));

    // RcとRefCellを組み合わせて複数の所有者が同じ値をいじる処理を作成する
    let value = Rc::new(RefCell::new(1));
    let a = Rc::clone(&value);
    let b = Rc::clone(&value);

    // どの変数で値を変更しても、すべての値に対して変更が反映される
    {
        *value.borrow_mut() += 10;
        println!("value = {:?}, a = {:?}, b = {:?}", &value, &a, &b);
    }
    {
        *a.borrow_mut() += 10;
        println!("value = {:?}, a = {:?}, b = {:?}", &value, &a, &b);
    }
    {
        *b.borrow_mut() += 10;
        println!("value = {:?}, a = {:?}, b = {:?}", &value, &a, &b);
    }    

    // 循環参照によるメモリリークの例
    let cyclic0 = Rc::new(Cyclic::Loop(RefCell::new(Rc::new(Cyclic::None))));
    let cyclic1 = Rc::new(Cyclic::Loop(RefCell::new(Rc::clone(&cyclic0))));
    *cyclic0.ret().borrow_mut() = Rc::clone(&cyclic1);
    
}

fn hello(msg: &str) {
    println!("Hello {}", msg);
}