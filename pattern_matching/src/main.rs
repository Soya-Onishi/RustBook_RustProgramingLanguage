fn main() {
    let robot_name = Some(String::from("Bob"));

    // refを外すとrobot_nameの所有権がmatch式の中に入ってしまうので、
    // 下の`println!("variable is {:?}", robot_name);`でコンパイルエラーが発生する。
    match robot_name {
        Some(ref name) => println!("name: {}", name),
        None                    => println!("no name"),
    }

    println!("variable is {:?}", robot_name);

    // @束縛の例
    let num = Some(10);
    match num {
        Some(x @ 0..=5) => println!("minimal number: {}", x),
        Some(x @ 6..=10) => println!("big number: {}", x),
        _ => println!("others"),
    }
}
