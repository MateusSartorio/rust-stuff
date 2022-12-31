use art_mateus_sartorio::{mix, PrimaryColor};

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    let result = mix(red, yellow);

    println!("{:?}", result);
}
