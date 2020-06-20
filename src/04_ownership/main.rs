fn main() {
    str_vs_string();
    mutate_string();
    move_ownership();
    copy_data();
    borrow_as_mutable();
    slice();
}

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn str_vs_string() {
    let s = "hello";
    print_typename(s);

    let s = String::from("hello");
    print_typename(s);
}

fn mutate_string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn move_ownership() {
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}", s2);
    // println!("{}", s1); // error! ownership was moved and the value  is dropped

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy
    println!("{}", s2);
    println!("{}", s1); // valid access because it was copied
}

fn copy_data() {
    let x = 5;
    let y = x;

    // they caused no errors because these values have no ownership
    // unlike types made on heap, type made on stack has known size. is is quick to make value with the size on stack
    // in the end, there is no reason why Rust prevent using data after copy
    // data types that implement Copy is just copied on stack
    println!("x = {}, y = {}", x, y);
}

fn borrow_as_mutable() {
    let mut s = String::from("hello");
    change(&mut s);

    // to borrow ownership does not drop value
    fn change(s: &mut String) {
        s.push_str(", world!")
    }
}

fn slice() {
    let string_literal = "first second";
    let string = String::from(string_literal);
    let w1 = first_word(string_literal);
    let w2 = first_word(&string);

    print_typename(w1);
    print_typename(&string[..]);
    print_typename(&string); // reference to Vecoter, that is slice

    println!("{}", w1);
    println!("{}", w2);

    // if parameter is (s: &String), it will not take string literals
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }
}
