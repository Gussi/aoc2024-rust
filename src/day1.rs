pub fn part1(input: &str) -> i32 {
    let (mut first_list_of_numbers, mut second_list_of_numbers) = get_list_of_numbers(input);

    first_list_of_numbers.sort();
    second_list_of_numbers.sort();

    let mut total_distance = 0;
    for (first, second) in first_list_of_numbers.iter().zip(second_list_of_numbers.iter()) {
        total_distance += (first - second).abs();
    }

    total_distance
}

#[test]
fn test_part1() {
    let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
    assert_eq!(part1(input), 11);
}

pub fn part2(input: &str) -> i32 {
    let (first_list_of_numbers, second_list_of_numbers) = get_list_of_numbers(input);

    let mut total_similarity = 0;
    for number in first_list_of_numbers.iter() {
        let occurences = second_list_of_numbers.iter().filter(|&n| n == number).count() as i32;
        total_similarity += number * occurences;
    }

    total_similarity
}

#[test]
fn test_part2() {
    let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
    assert_eq!(part2(input), 31);
}

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
