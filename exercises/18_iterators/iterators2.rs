fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars(); // array of chars
    let mut result: String = String::new();
    match chars.next() {
        None => return result,
        Some(first) => {
            result += &first.to_uppercase().to_string();
        },
    }
    result += chars.as_str();
    result
}

/*
 When you call `collect()`, Rust looks at the surrounding context to infer the type of the collection you want to create. 
If the function signature specifies a return type, Rust uses that information to determine what type to collect into.
*/

fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    //words.iter().map(|word| capitalize_first(word)).collect()

    let mut result:Vec<String> = Vec::new();
    for word in words.iter(){
        result.push(capitalize_first(word))
    }
    result
}

fn capitalize_words_string(words: &[&str]) -> String {
    //words.iter().map(|word| capitalize_first(word)).collect()
    let mut result: String= String::new(); 
    for word in words.iter() {
        result += &capitalize_first(word);
    }
    result
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
