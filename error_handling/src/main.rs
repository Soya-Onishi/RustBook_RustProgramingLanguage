use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    // simple_panic();
    // panic_by_out_of_bounds();
    // open_file();
    delegate_error();
}

fn simple_panic() {
    panic!("crash and burn");
}

fn panic_by_out_of_bounds() {
    let v = vec![1, 2, 3];

    v[99];
}

fn open_file() {
    let f = File::open("./hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(err) => panic!("There was a problem opening the file: {:?}", err)
    };    
}

fn delegate_error() -> Result<String, io::Error>{
    // ?によってErrの値はFrom<io::Error>トレイトによって変換される
    // From<io::Error>トレイトが実装されていない型のErrが存在する場合、
    // 実装してあげる必要がある。
    let mut f = File::open("./hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)    
}