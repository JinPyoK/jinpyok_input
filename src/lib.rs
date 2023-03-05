//! My input collection

use std::io::stdin;

// When input multiple data, return Vec<String>.. last string probably has '\n'
fn input_multiple(comment: &str) -> Vec<String> {
    println!("{}", comment);
    let mut tmp = String::new();
    match stdin().read_line(&mut tmp) {
        Ok(_) => {},
        Err(e) =>println!("input_s error: {}", e),
    }
    let result : Vec<String> = tmp.split_whitespace().map(|s| s.to_string()).collect();
    result
}

// When input one string
fn input_string(comment: &str) -> String {
    println!("{}", comment);
    let mut tmp = String::new();
    match stdin().read_line(&mut tmp) {
        Ok(_) => {},
        Err(e) =>println!("input_s error: {}", e),
    }
    let result : String = tmp.trim().to_string();
    result
}

// when input one of i8 ~ i64, u8 ~ u64, isize, usize
fn input_s<T: std::str::FromStr>(comment: &str) -> T {
    println!("{}", comment);
    let mut tmp = String::new();
    match stdin().read_line(&mut tmp) {
        Ok(_) => {},
        Err(e) =>println!("input_s error: {}", e),
    }
    let result : Option<T> =  match tmp.trim().parse() {
        Ok(n) => Some(n),
        Err(_) => None,

    };
    result.unwrap()
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test1 : i64 = input_s::<i64>("i64: ");
        let test2 : f32 = input_s::<f32>("f32: ");
        let test3 : i8 = input_s::<i8>("i8: ");
        let test4 : isize = input_s::<isize>("isize: ");
        let test5 : String = input_string("string: ");
        let test6 : Vec<String> = input_multiple("multiple: ");
        let temp : Vec<String> = vec!["abc".to_string(), "def".to_string()];
        assert_eq!(1, test1);
        assert_eq!(2.0, test2);
        assert_eq!(3, test3);
        assert_eq!(4, test4);
        assert_eq!("abcde".to_string(), test5);
        assert_eq!(temp, test6);
    }
}
