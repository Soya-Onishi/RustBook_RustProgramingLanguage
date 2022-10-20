use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // moveにより、別スレッドに`v`の所有権を委譲
    let handle = thread::spawn(move || {
        for i in v.iter() {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(100));
        }        
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(100));
    }

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![1, 2, 3, 4];

        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for received in rx {
        println!("Got Numbers: {}", received);
    }

    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![10, 20, 30, 40];
        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }        
    });

    thread::spawn(move || {
        let vals = vec![5, 15, 25, 35];
        for v in vals {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for received in rx {
        println!("Got Numbers from multiple Tx: {}", received);
    }

    // RcにはSendトレイトの実装がないのでスレッド間で所有権のmoveができない。
    // Arcを代わりに使用することで可能になる。
    let counter  = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }    

    println!("count = {}", counter.lock().unwrap());
    
    // Rc      <-> Arc
    // RefCell <-> Mutex
    // RefCellはSyncがないため、スレッド間での借用は安全ではない。    
}
