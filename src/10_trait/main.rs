use the_book::lib_10::{largest, notify, NewsArticle, Summary, Tweet};

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2: Point<f32> = Point { x: 5.0, y: 10.0 };
    println!("x: {}", p1.x());
    println!("distance: {}", p2.distance_from_origin());

    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: "Penguins win the Stanley Cup Championship!".to_string(),
        location: "Pittsburgh, PA, USA".to_string(),
        author: "Iceburgh".to_string(),
        content: "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL."
            .to_string(),
    };
    notify(&article);
    notify(&tweet);

    let number_list = vec![1, 7, 5, 9, 3];
    let largest_number = largest(&number_list);
    println!("The largest number is: {}", largest_number);

    let char_list = ['y', 'm', 'a', 'q'];
    let largest_char = largest(&char_list);
    println!("The largest char is: {}", largest_char);

    let x = "great".to_string();
    // let result;
    {
        let y = "awesome".to_string();
        let longest_char = longest(x.as_str(), y.as_str());
        println!("The longest char is: {}", longest_char);

        // error! voilates a lifetime constraint of the function
        // the lifetime of y is smaller than the one of result
        // result = longest(x.as_str(), y.as_str());
    }
    // println!("The longest char is: {}", result);
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// like Type Class
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
