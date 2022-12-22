enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

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

    let mut s = String::from("cool bro");
    s.push_str("bar");
}
