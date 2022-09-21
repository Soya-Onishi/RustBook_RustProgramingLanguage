enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4, 5];
        let third = &v[2];
        println!("The third element {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None              => println!("There is no third element"),
        }
    }
    
    /*
    v.push(6)で可変と不変参照の共存によりコンパイルエラー
    {
        let mut v = vec![1, 2, 3, 4, 5];
        let elem = &v[2];        
        v.push(6);        
        println!("The third element is {}", elem);
    }
    */

    // for文による全要素更新と全要素走査
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }

        for i in &v {
            println!("{}", i);
        }
    }

    // Enum型を要素としたVec
    {    
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }

    {
        let data = "initial contents";
        let s = data.to_string();
        println!("{}", s);
        let s = "initial contents".to_string();
        println!("{}", s);
    }

    // 文字列の更新
    {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("{}", s);
        
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("{},{}", s1, s2);        

        let mut s = String::from("lo");
        s.push('l');
        println!("{}", s);
    }

    // formatによる文字列の結合
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);

        println!("{}", s);
    }

    // 文字列のスライス
    {
        let hello = "Здравствуйте";
        let s1 = &hello[0..4];

        // 各文字のバイト長と合わないためpanic
        // let s2 = &hello[0..1];

        println!("{}", s1);
    }

    // 文字列の走査
    {
        println!("print chars");
        let msg = "नमस्ते";
        for c in msg.chars() {
            println!("{}", c);
        }

        println!("print bytes");
        for b in msg.bytes() {
            println!("{}", b);
        }
    }
}
