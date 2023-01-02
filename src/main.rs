fn main() {
    test_if_let();
    test_while_let();
    test_for();
    print_coordinates(&(3, 4));

    test_match();
    test_variable();

    test_struct();
    test_enum();
}

struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move{ x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

pub fn test_if_let() {
    let favorite_color: Option<&str> = Some("red");
    let is_tuesday: bool = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

pub fn test_while_let() {
    let mut stack = Vec::new();
    stack.push("hi");
    stack.push("111");
    stack.push("222");
    stack.push("333");

    while let Some(val) = stack.pop() {
        println!("pop: {}", val);
    }
}

pub fn test_for() {
    let mut stack = Vec::new();
    stack.push("hi");
    stack.push("111");
    stack.push("222");

    for (index, value) in stack.iter().enumerate() {
        println!("for: {}, {}", value, index);
    }
}

pub fn print_coordinates(&(x, y): &(u32, u32)) {
    println!("print_coordinates : ({}, {})", x, y);
}

pub fn test_match() {
    let x = 10;
    match x {
        1 | 2 => println!("x is 1 or 2"),
        3 => println!("x is 3"),
        // ..= 语法允许你匹配一个闭区间范围内的值, 范围只允许用于数字或 char 值,因为编译器会在编译时检查范围不为空
        6..=10 => println!("x is 6 - 10"),
        _ => println!("anything"),
        /*
        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
        */
    }
}

pub fn test_variable() {
    let x = None;
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

pub fn test_struct() {
    let p = Point { x: 10, y: 20 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point {x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

pub fn test_enum() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }, 
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }, 
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),

        Message::ChangeColor(Color::Hsv(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}
