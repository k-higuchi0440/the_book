use std::option::Option;

fn main() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    // println!("can't use y here: {:?}", y);

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    // println!("can't use v1_iter here: {:?}", v1_iter);

    assert_eq!(total, 6);

    calling_next_directly();
    using_other_iterator_methods();
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn calling_next_directly() {
    let mut counter = Counter::new();
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
}

fn using_other_iterator_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("{}", sum);
}
