use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use List::{Cons, Nil};

fn main() {
    box_list();
    println!("--------------------------------");
    my_box();
    println!("--------------------------------");
    custom_smart_pointer();
    println!("--------------------------------");
    rc_list();
    println!("--------------------------------");
    rc_refcell_list();
    println!("--------------------------------");
    weak_ref();
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn box_list() {
    // this value is allocated on the heap
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}

#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn rc_list() {
    let a = Rc::new(RcList::Cons(
        5,
        Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = RcList::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = RcList::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

#[derive(Debug)]
enum RcCellList {
    Cons(Rc<RefCell<i32>>, Rc<RcCellList>),
    Nil,
}

fn rc_refcell_list() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(RcCellList::Cons(
        Rc::clone(&value),
        Rc::new(RcCellList::Nil),
    ));
    let b = RcCellList::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = RcCellList::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn my_box() {
    let x = 5;
    let y = MyBox::new(x);

    println!("5 == x: {}", 5 == *y);
    println!("5 == *y: {}", 5 == *y);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn weak_ref() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

fn custom_smart_pointer() {
    let b = CustomSmartPointer {
        data: String::from("first stuff"),
    };
    let _c = CustomSmartPointer {
        data: String::from("secode stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("third stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(b);
    println!("this is the end of fucntion.");
}
