enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

/*
enum Option<T> {
    None,
    Some(T),
}
*/

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    /*
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    */
    /*
    let home = IpAddrKind::V4(String::from(127, 0, 0, 1));
    let loopback = IpAddrKind::V6(String::from("::1"));
    */
    /*
    let m = Message::Write(String::from("hello"));
    m.call();
    */
    
    /*
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    */
    /*
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(none);
    */

    let config_max = Some(3u8);

    /*
    match config_max {
        Some(max) => println("The maximum is configured to be {}", max),
        _ => (),
    }
    */

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } 
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
} 

/*
fn route(ip_kind: IpAddrKind) {

}
*/
