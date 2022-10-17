use std::thread;
use std::time::Duration;

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
}
