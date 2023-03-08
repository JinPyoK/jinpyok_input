//! My input collection

use std::io::stdin;

// When input multiple data, return Vec<String>.. last string probably has '\n'
pub fn input_multiple(comment: &str) -> Option<Vec<String>> {
    println!("{}", comment);
    let mut result: String = String::new();
    match stdin().read_line(&mut result) {
        Ok(_) => Some(result.split_whitespace().map(|s| s.to_string()).collect()),
        Err(e) => {
            println!("input_multiple read_line error: {}", e);
            None
        }
    }
}

// When input one string
pub fn input_string(comment: &str) -> Option<String> {
    println!("{}", comment);
    let mut result: String = String::new();
    match stdin().read_line(&mut result) {
        Ok(_) => Some(result.trim().to_string()),
        Err(e) => {
            println!("input_string read_line error: {}", e);
            None
        }
    }
}

// when input one of i8 ~ i64, u8 ~ u64, isize, usize
pub fn input_s<T: std::str::FromStr>(comment: &str) -> Option<T> {
    println!("{}", comment);
    let mut result: String = String::new();
    match stdin().read_line(&mut result) {
        Ok(_) => {}
        Err(e) => {
            println!("input_s read_line error: {}", e);
            return None;
        }
    }
    match result.trim().parse() {
        Ok(n) => Some(n),
        Err(_) => {
            println!("input_s parse error");
            None
        }
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test1: i64 = input_s::<i64>("i64: ").unwrap();
        let test2: f32 = input_s::<f32>("f32: ").unwrap();
        let test3: i8 = input_s::<i8>("i8: ").unwrap();
        let test4: isize = input_s::<isize>("isize: ").unwrap();
        let test5: String = input_string("string: ").unwrap();
        let test6: Vec<String> = input_multiple("multiple: ").unwrap();
        let temp: Vec<String> = vec!["abc".to_string(), "def".to_string()];
        assert_eq!(1, test1);
        assert_eq!(2.0, test2);
        assert_eq!(3, test3);
        assert_eq!(4, test4);
        assert_eq!("abcde".to_string(), test5);
        assert_eq!(temp, test6);
    }
}
