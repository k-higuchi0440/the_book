use std::slice;

fn main() {
    unsafe_features();
    println!("----------------------");
    advanced_trait();
    println!("----------------------");
    advanced_functions_and_closures();
    println!("----------------------");
    macros();
}

fn unsafe_features() {
    // you can create mutable and immutable raw pointers at once
    // which is not allowed by ownership rules
    let mut num = 5;
    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer

    // but you cannot dereference out of safe block
    // println!("{}", *r1);

    unsafe {
        println!("r1 is:  {}", *r1);
        println!("r2 is:  {}", *r2);
    }

    // call unsafe function
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (former, latter) = split_at_mut(&mut v, 3);
    println!("{:?}", former);
    println!("{:?}", latter);

    unsafe {
        println!("Absolute value of -3 is according to C: {}", abs(-3));
    }

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}

unsafe fn _invalid_memory_access() {
    let address = 0x01234usize;
    let r = address as *mut i32; // mutable raw pointer
    let slice: &[i32] = slice::from_raw_parts_mut(r, 10000);
    println!("{:?}", slice);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // mutable raw pointer

    assert!(mid <= len);

    // create a slice from pointer to a slice and length of the slice
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// ABI(Application Binary Interface)
extern "C" {
    fn abs(input: i32) -> i32;
}

// it would result in a race condition with multi threads
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {}
unsafe impl Foo for i32 {}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn advanced_trait() {
    let p = Point { x: 1, y: 0 } + Point { x: 2, y: 3 };
    println!("{:?}", p);

    let m = Millimeters(50) + Meters(10);
    println!("{:?}", m);

    let person = Human;
    person.fly(); // call a method in Human impl block by default
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("the baby dog is called {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    p.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

use std::ops::Add;

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(i32);
struct Meters(i32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    // associated function
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// new type pattern without overhead at runtime
// because the wrapper type is elided at compile time
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// type alias can have type parameters
type _Result<T> = std::result::Result<T, std::io::Error>;

// the generic type may be or may not be sized.
fn _generic<T: ?Sized>(_t: &T) {}

fn advanced_functions_and_closures() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    // function pointer implements Fn, FnMat and FnOnce
    // you can pass fn to a function argument like map
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("{:?}", list_of_strings);

    // you can pass tuple-struct enum variants as function arguments
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// fn type is called a function pointer.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
}

// closure is traits. so you can't return fn directly
fn _returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn macros() {
    // 1. decralative macros
    let _v = my_vec![1, 2, 3];
    // the above macro will generate the below code
    // {
    //   let mut temp_vec = Vec::new();
    //   temp_vec.push(1);
    //   temp_vec.push(2);
    //   temp_vec.push(3);
    //   temp_vec
    // }

    // 2. procedural macros
    Pancakes::hello_macro();
}

// 1. decralative macros
// macro_rules! will be deprecated in the future
#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// 2. procedural macros
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

trait HelloMacro {
    fn hello_macro();
}

// 3. attribute-like macros
// attiributes can be applied to other items as well, such as functions
//
// #[route(GET, "/")]
// fn index() {
//
// attr: GET, "/"
// item: fn index {}
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

// 4. function-like macros
// macro_rules! allows us to match patterns.
// you can do more complex tasks with this macro.
//
// let sql = sql!(SELECT * FROM posts WHERE id=1);
//
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {
