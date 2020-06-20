fn main() {
    // variables
    println!("mutate");
    mutate();

    println!("shadow");
    shadow();

    // data types
    // access to array with invalid index
    // let a = [1, 2, 3];
    // let index = 10;
    // println!("{}", a[index]);

    // functions
    let x = return_five();
    println!("{}", x);

    break_loop_with_value();
}

fn mutate() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadow() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len(); // redeclare with a different type
    println!("it has {} spaces", spaces);
}

fn return_five() -> i32 {
    5
}

fn break_loop_with_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Count is {}", counter);
    println!("The result is {}", result);
}
