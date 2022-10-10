use std::fmt::Display;

fn main() {
    sample1();
    sample2();   
    sample3();
    sample3_exercise();
    struct_generics();

    sample_lifetime1();
    sample_lifetime2();
}

fn sample1() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("maximum number is {}", largest);
}

fn sample2() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number;
        }
    }    

    largest
}

fn sample3() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_generics(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_generics(&char_list);
    println!("The largest char is {}", result);
}

fn sample3_exercise() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_generics_shared(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_generics_shared(&char_list);
    println!("The largest char is {}", result);
}

fn largest_generics<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_generics_shared<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    #[allow(dead_code)]
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn struct_generics() {
    let point = Point { x: 5, y: 4 };

    // xとyで型が異なるためコンパイルエラー
    // let wont_work = Point { x: 5, y: 4.0 };    

    println!("x: {}", point.x());
    
}

pub trait Summary {
    fn summarize(&self) -> String;
}

// impl SummaryはT: Summaryの糖衣構文
#[allow(dead_code)]
fn notify(item: &(impl Summary + Display)) {
    println!("{}, {}", item, item.summarize());
}

#[allow(dead_code)]
fn using_where_clause<T>(item: &T) -> T
    where T: Display + Summary + Clone
{
    item.clone()
}

fn sample_lifetime1() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_lifetime(string1.as_str(), string2);

    println!("The longest string is {}", result);
}

fn sample_lifetime2() {
    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest_lifetime(string1.as_str(), string2.as_str());

        println!("The longest string is {}", result);
    }
    // ライフタイムの制約によりコンパイルエラー
    // println!("The longest string is {}", result);
}

fn longest_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

