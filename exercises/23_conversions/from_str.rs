// This is similar to the previous `from_into` exercise. But this time, we'll
// implement `FromStr` and return errors instead of falling back to a default
// value. Additionally, upon implementing `FromStr`, you can use the `parse`
// method on strings to generate an object of the implementor type. You can read
// more about it in the documentation:
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<u8>()
    ParseInt(ParseIntError),
}

impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string_vec: Vec<String> = s.split(',').map(|s| s.to_string()).collect();
        
        if string_vec.len() == 2 {
            if !string_vec[0].is_empty() {
                match string_vec[1].parse::<u8>() {
                    Ok(age_result) => {
                        Ok(self::Person {
                            name: string_vec[0].clone(),
                            age: age_result
                        })
                    }
                    Err(e)=> Err(self::ParsePersonError::ParseInt(e))    
                }
            } else {
                Err(self::ParsePersonError::NoName)
            }

        } else {
            Err(self::ParsePersonError::BadLen)
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
