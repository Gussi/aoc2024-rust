/// Contains both parts of the problem.
pub mod part {

    /// Solves the first part of the problem.
    pub fn one (input: &str) -> i32 {
        let (mut first_list_of_numbers, mut second_list_of_numbers) = get_list_of_numbers(input);

        first_list_of_numbers.sort();
        second_list_of_numbers.sort();

        let mut total_distance = 0;
        for (first, second) in first_list_of_numbers.iter().zip(second_list_of_numbers.iter()) {
            total_distance += (first - second).abs();
        }

        total_distance
    }

    /// Solves the second part of the problem.
    pub fn two(input: &str) -> i32 {
        let (first_list_of_numbers, second_list_of_numbers) = get_list_of_numbers(input);

        let mut total_similarity = 0;
        for number in first_list_of_numbers.iter() {
            let occurences = second_list_of_numbers.iter().filter(|&n| n == number).count() as i32;
            total_similarity += number * occurences;
        }

        total_similarity
    }

    /// Parses the input and returns two lists of numbers, left and right.
    fn get_list_of_numbers(input: &str) -> (Vec<i32>, Vec<i32>) {
        let mut first_list_of_numbers = Vec::new();
        let mut second_list_of_numbers = Vec::new();

        for line in input.lines() {
            let mut numbers = line.split("   ");
            first_list_of_numbers.push(numbers.next().unwrap().parse::<i32>().unwrap());
            second_list_of_numbers.push(numbers.next().unwrap().parse::<i32>().unwrap());
        }

        (first_list_of_numbers, second_list_of_numbers)
    }

    /// The input for the tests.
    #[cfg(test)]
    const TEST_INPUT: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    /// Tests the first part of the problem.
    #[test]
    fn test_one() {
        assert_eq!(one(TEST_INPUT), 11);
    }

    /// Tests the second part of the problem.
    #[test]
    fn test_two() {
        assert_eq!(two(TEST_INPUT), 31);
    }
}
