fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use crate::is_even;
    #[test]
    fn you_can_assert() {
        assert!(is_even(6));
        assert!(!is_even(9));
    }
}
