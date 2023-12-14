#[allow(dead_code)]
pub fn get_token() -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token() {
        assert_eq!(get_token(), 42);
    }
}
