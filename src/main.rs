#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("called with {:?}", self);
    }

    fn call_all(collection: Vec<Message>) {
        for c in collection {
            c.call();
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(val) => Some(val + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn if_let_control_flow() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // this block is equivalent to below, but much better
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // this block is equivalent to below, but much better
    let mut count = 0;
    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    println!("{:?}", four);
    println!("{:?}", six);

    let r = Message::Write(String::from("some"));
    let q = Message::Quit;
    let m = Message::Move { x: 6, y: 7 };
    let c = Message::ChangeColor(23, 56, 76);
    r.call();
    q.call();
    m.call();
    c.call();

    Message::call_all(vec![r, q, m, c]);

    let some_number = Some(5);
    println!("some_number is some: {}", some_number.is_some());
    let some_string = Some("s string");
    println!("some_string is some: {}", some_string.is_some());

    let absent_number: Option<i32> = None;
    println!("absent_number is none: {}", absent_number.is_none());

    let cents = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("cents: {}", cents);

    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    let none = plus_one(None);
    println!("{:?}", none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}
