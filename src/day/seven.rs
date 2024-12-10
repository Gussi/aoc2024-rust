pub mod part {
    pub fn one(_input: &str) -> i32 {
        0
    }

    pub fn two(_input: &str) -> i32 {
        0
    }

    #[cfg(test)]
    const TEST_INPUT: &str = "";

    #[test]
    pub fn test_one() {
        assert_eq!(one(TEST_INPUT), 0);
    }

    #[test]
    pub fn test_two() {
        assert_eq!(two(TEST_INPUT), 0);
    }
}
