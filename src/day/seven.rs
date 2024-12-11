pub mod part {

    /// Solve part one of the challenge
    pub fn one(input: &str) -> usize {
        Equation::from_input(input)
            .iter()
            .filter(|equation| equation.solvable(&[Operators::Add, Operators::Multiply]))
            .map(|equation| equation.answer)
            .sum()
    }

    /// Solve part two of the challenge
    pub fn two(_input: &str) -> usize {
        Equation::from_input(_input)
            .iter()
            .filter(|equation| equation.solvable(&[Operators::Add, Operators::Multiply, Operators::Concatenate]))
            .map(|equation| equation.answer)
            .sum()
    }

    #[derive(Clone, Copy)]
    enum Operators {
        Add,
        Multiply,
        Concatenate,
    }

    struct Equation {
        answer: usize,
        numbers: Vec<usize>,
    }

    impl Equation {

        fn from_input(input: &str) -> Vec<Self> {
            input.lines().map(|line| {
                let mut parts = line.split(": ");
                let answer = parts.next().unwrap().parse().unwrap();
                let numbers = parts.next().unwrap().split(" ").map(|n| n.parse().unwrap()).collect();
                Self { answer, numbers }
            }).collect()
        }

        fn solvable(&self, operators: &[Operators]) -> bool {
            generate_operator_combinations(self.numbers.len() - 1, operators)
                .iter()
                .any(|operation| self.solve(operation.clone()) == self.answer)
        }

        fn solve(&self, operators: Vec<Operators>) -> usize {
            let mut result = self.numbers[0];

            for (i, operator) in operators.iter().enumerate() {
                match operator {
                    Operators::Add => result += self.numbers[i + 1],
                    Operators::Multiply => result *= self.numbers[i + 1],
                    Operators::Concatenate => {
                        let first = result.to_string();
                        let second = self.numbers[i + 1].to_string();

                        result = format!("{}{}", first, second).parse().unwrap();
                    }
                }
            }

            result
        }
    }

    fn generate_operator_combinations(slots: usize, possible_ops: &[Operators]) -> Vec<Vec<Operators>> {
        if slots == 0 {
            return vec![vec![]];
        }

        if slots == 1 {
            return possible_ops.iter().map(|&op| vec![op]).collect();
        }

        let mut results = Vec::new();
        let smaller_combos = generate_operator_combinations(slots - 1, possible_ops);
        for combo in smaller_combos {
            for &op in possible_ops {
                let mut new_combo = combo.clone();
                new_combo.push(op);
                results.push(new_combo);
            }
        }

        results
    }

    #[cfg(test)]
    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    pub fn test_one() {
        assert_eq!(one(TEST_INPUT), 3749);
    }

    #[test]
    pub fn test_two() {
        assert_eq!(two(TEST_INPUT), 11387);
    }
}
