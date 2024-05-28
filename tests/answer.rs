pub fn answer() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*; // Import all the names from the outer module

    #[test]
    fn check_answer_validity() {
        assert_eq!(answer(), 42);
    }
}
