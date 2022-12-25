// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1,
// }

// impl<X1, Y1> Point<X1, Y1> {
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// mod aggregator;
// use aggregator::{Summary, Tweet, NewsArticle, notify};

// fn main() {
    // let number_list = vec!['a', 'b'];

    // let result = largest(&number_list);

    // println!("The largest number is {}", result);

    // // let wont_work = Point { x: 5, y: 4.0 };
    // let _integer_and_float = Point { x: 5, y: 4.0 };
    // println!("{:?}", _integer_and_float);

    // let p = Point { x: 5, y: 10 };
    // println!("p.x = {}", p.x());

    // let p1 = Point { x: 5, y: 10.4 };
    // let p2 = Point { x: "Hello", y: 'c' };

    // let p3 = p1.mixup(p2);

    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // let tweet = Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from("of course, as you probrably already know people"),
    //     reply: false,
    //     retweet: false,
    // };

    // println!("1 new tweet: {}", tweet.summarize());

    // let article = NewsArticle {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    // };

    // // println!("New article available! {}", article.summarize());

    // notify(&article);
    // println!("New article available! {}", article.summarize());


// }

// fn main() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {}", r);
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

// &i32
// &'a i32
// &'a mut i32

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}