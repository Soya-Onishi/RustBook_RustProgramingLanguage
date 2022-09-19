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
}
