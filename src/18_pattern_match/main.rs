fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
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

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (_x, _y, _z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3..=10 => println!("three through 10"),
        _ => println!("anything"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("{}", x);
    println!("{}", y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On meither axis at ({}, {})", x, y),
    }

    let color = Color { r: 0, g: 0, b: 0 };
    match color {
        Color {
            r: red @ 10..=12, ..
        } => println!("red is {}", red),
        Color { r, .. } => println!("red is {}", r),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("Some numbers: {}, {}", first, last),
    }

    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = 4;
    let y = false;
    match x {
        // 4 | 5 | 6 AND y. so x does not match this case
        4 | 5 | 6 if y => println!("yes!"),
        _ => println!("no"),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
struct Color {
    r: u32,
    g: u32,
    b: u32,
}
