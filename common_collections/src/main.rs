use std::collections::HashMap;

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

    // 新規ハッシュマップの作成
    {            
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        scores.insert(String::from("Red"), 100);
    }

    // タプルのベクタからハッシュマップを生成する
    {        
        let teams = vec![String::from("Blue"), String::from("Yellow"), String::from("Red")];
        let init_scores = vec![10, 50, 100];
        
        let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

        println!("hashmap example: {:?}", scores);
    }

    // キーやバリューの所有権
    {     
        let field_name = String::from("Favorite_color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);

        // 所有権がmapに渡っているので、コンパイルエラーになる
        // println!("{}, {}", field_name, field_value);
    }
    
    // ハッシュマップの値にアクセスする
    {                
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);        

        let team_name = String::from("Blue");
        let score = scores.get(&team_name).map(|v| *v).unwrap_or(0);
        
        println!("{}: {}", &team_name, score);
    }

    // ハッシュマップにバリューがないときのみ挿入
    {     
        let mut scores = HashMap::new();
        scores.entry(String::from("Blue")).and_modify(|v| *v = *v + 10).or_insert(50);

        println!("{:?}", scores);
    }

    // ハッシュマップの操作を利用した単語のカウント
    {        
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            map.entry(word).and_modify(|v| *v = *v + 1).or_insert(1);
        }

        println!("{:?}", map);
    }

    // 練習問題1: ベクタを使ってmean, median, modeを求める
    {
        let mut nums: Vec<i32> = vec![1, 9, 5, 4, 9, 3, 8, 5, 6, 9];
        nums.sort();
        
        let sum: i32 = nums.iter().sum();
        let average = sum as f32 / nums.len() as f32;

        let median = 
            if nums.len() % 2 == 0 {
                let idx = nums.len() / 2 - 1;
                (nums[idx] + nums[idx + 1]) as f32 / 2_f32
            } else {
                let idx = nums.len() / 2;
                nums[idx] as f32
            };
        
        let mut count = HashMap::new();
        for &n in nums.iter() {
            count.entry(n).and_modify(|v| *v = *v + 1).or_insert(1);
        }

        let max = count.iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2));

        println!("average: {}", average);
        println!("median: {}", median);
        if let Some((k, v)) = max {
            println!("max: ({}, {})", k, v)
        }
    }

    // 練習問題2: ビッグラテンへの変換
    {
        let text = "first";
        let mut cs: Vec<_> = text.chars().collect();
        let first = cs.remove(0);
        let big_laten = format!("{}-{}ay", cs.iter().collect::<String>(), first);

        println!("{}", big_laten);
    }

    // 練習問題3: ハッシュマップによる雇用者管理
    {                 
        let add = |k: String, v: String, map: &mut HashMap<String, Vec<String>>| {
            let mut vec = map.entry(k).or_insert(vec![]);
            vec.push(v);
        };

        let show_department_employee = |department: &String, map: &HashMap<String, Vec<String>>| {            
            let employees = if let Some(employees) = map.get(department) {
                employees.clone()
            } else {
                Vec::new()
            };

            println!("{:?}", employees);
        };

        let show_all_employee = |map: &HashMap<String, Vec<String>>| {
            for (k, v) in map.iter() {
                println!("department: {}, employees: {:?}", k, v);
            }
        };

        let mut map = HashMap::new();
        add(String::from("Engineering"), String::from("Sally"), &mut map);
        add(String::from("Sales"), String::from("Amir"), &mut map);
        show_department_employee(&String::from("Engineering"), &map);
        show_department_employee(&String::from("Sales"), &map);
        show_all_employee(&map);
    }
}
