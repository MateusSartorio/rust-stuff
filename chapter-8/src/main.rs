// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

use std::collections::HashMap;

fn main() {
    // let v: Vec<i32> = Vec::new();

    // let mut v = vec![1, 2, 3];
    // v.push(5);
    // v.push(6);

    // let mut v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // println!("The third element is {third}");

    // let second: Option<&i32> = v.get(1);
    // match second {
    //     Some(second) => println!("The second element is {second}"),
    //     None => println!("There is no third element."),
    // }

    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12);
    // ];

    // let mut s = String::new();
    
    // let data = "initial contents";
    // let s = data.to_string();
    // let s2 = "initial contents".to_string();

    // let s = String::from("cool bro");

    // let mut s = String::from("cool bro");
    // s.push_str("bar");

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {s2}");

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{s1}-{s2}-{s3}");

    // println!("{}", s);

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);

    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // println!("{field_name}");

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);

    // println!("{:?}", scores);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
