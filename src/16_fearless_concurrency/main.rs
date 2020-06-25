use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    join_thread();
    move_closure();
    send_value();
    send_values();
    multiple_producers_single_consumer();
    mutex_auto_unlock();
    atomically_reference_counted();
}

fn join_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // block main thread
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn move_closure() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

fn send_value() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("single educk");
        tx.send(val).unwrap();
    });

    // block main thread
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn send_values() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("duck"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // block main thread
    for received in rx {
        println!("Got: {}", received);
    }
}

fn multiple_producers_single_consumer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("tx1: duck"),
            String::from("tx1: from"),
            String::from("tx1: the"),
            String::from("tx1: thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("tx2: more"),
            String::from("tx2: messages"),
            String::from("tx2: for"),
            String::from("tx2: you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // block main thread
    for received in rx {
        println!("Got: {}", received);
    }
}

fn mutex_auto_unlock() {
    let m = Mutex::new(5);
    {
        // block main thread
        let mut num = m.lock().unwrap();
        *num = 6;
    } // unlock and drop automatically!
    println!("m = {:?}", m);
}

fn atomically_reference_counted() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
