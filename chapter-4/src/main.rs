fn main() {
    /*
    let s = String::from("hello world!");

    let hello = &s[0..5];
    let hello2 = &s[..5];

    let world = &s[6..11];
    let world2 = &s[6..];

    let s2 = &s[..];
    */
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

/*
fn dangle() -> &String {
    static s = String::from("hello");

    &s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
*/
