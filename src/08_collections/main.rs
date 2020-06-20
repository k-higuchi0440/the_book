use std::collections::HashMap;

fn main() {
    // vector
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    println!("{:?}", v);

    // move ownership
    let v = v;
    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is not third element.")
    }

    // string
    let s1 = "Hello, ".to_string();
    let s2 = "world!".to_string();
    let s3 = s1 + &s2; // s1's ownership is moved
    println!("{}", s3);

    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let multi_bytes = "नमस्ते";

    // UTF-8 scalar values
    // grapheme clusters are not supported in standard libraries
    for c in multi_bytes.chars() {
        print!("{}, ", c);
    }
    println!();

    // UTF-8 bytes
    for c in multi_bytes.bytes() {
        print!("{}, ", c);
    }
    println!();

    // HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // teams and initial_scores are moved
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);

    println!("get with Blue: {:?}", scores.get("Blue"));

    scores.insert(String::from("Blue"), 100);
    scores.entry(String::from("Yellow")).or_insert(200);

    for (team, score) in &scores {
        println!("{}: {}", team, score);
    }

}
