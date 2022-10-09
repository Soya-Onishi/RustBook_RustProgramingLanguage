use std::fs::File;

fn main() {
    // simple_panic();
    // panic_by_out_of_bounds();
    open_file();
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