enum IpAddrKind {
    // V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// impl Message {
//     fn some_fn() {
//         println!("some_fn");
//     }
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    // // let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // // option enum
    // enum option<T> {
    //     Some(T),
    //     None,
    // }

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y.unwrap_or(0);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
// fn route(ip_kind: IpAddrKind) {}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None => None,
        Some(i) => Some(i + 1),
        _ => None,
    }
}
