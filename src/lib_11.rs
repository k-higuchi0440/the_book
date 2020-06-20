pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[allow(dead_code)]
fn greeting(name: &str) -> String {
    format!("Hello!, {}", name)
}

#[allow(dead_code)]
fn cause_panic() {
    panic!("do not call this function. it's the only way")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = super::greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {}",
            result
        );
    }

    #[test]
    #[should_panic(expected = "do not call this function. it's the only way")]
    fn cause_panic() {
        super::cause_panic();
    }

    #[test]
    fn return_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("two plus two does not equal four".to_string())
        }
    }
}
