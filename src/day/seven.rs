pub mod part {
    pub fn one(input: &str) -> usize {
        let equations = parse_input(input);
        let mut total = 0;

        for equation in equations {
            if equation.solvable_part_one() {
                total += equation.answer;
            }
        }

        total
    }

    pub fn two(_input: &str) -> usize {
        let equations = parse_input(_input);
        let mut total = 0;

        for equation in equations {
            if equation.solvable_part_two() {
                total += equation.answer;
            }
        }

        total
    }

    #[derive(Clone, Copy, Debug)]
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
        fn new_from_input(input: &str) -> Self {
            let mut parts = input.split(": ");
            let answer = parts.next().unwrap().parse().unwrap();
            let numbers = parts.next().unwrap().split(" ").map(|x| x.parse().unwrap()).collect();

            Equation { answer, numbers }
        }

        fn solvable_part_one(&self) -> bool {
            let operations = generate_operator_combinations_part_one(self.numbers.len() - 1);

            for operation in operations {
                if self.solve(operation) == self.answer {
                    return true;
                }
            }

            false
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

        fn solvable_part_two(&self) ->bool {
            let operations = generate_operator_combinations_part_two(self.numbers.len() - 1);

            for operation in operations {
                if self.solve(operation.clone()) == self.answer {
                    return true;
                }
            }

            false
        }
    }

    fn generate_operator_combinations_part_two(slots: usize) -> Vec<Vec<Operators>> {
        if slots == 0 {
            return vec![vec![]];
        }

        let mut combos = vec![vec![Operators::Add], vec![Operators::Multiply], vec![Operators::Concatenate]];
        let mut results = Vec::new();

        for _ in 1..slots {
            let mut new_combos = Vec::new();

            for c in &combos {
                let mut add_combo = c.clone();
                add_combo.push(Operators::Add);
                new_combos.push(add_combo);

                let mut mul_combo = c.clone();
                mul_combo.push(Operators::Multiply);
                new_combos.push(mul_combo);

                let mut concat_combo = c.clone();
                concat_combo.push(Operators::Concatenate);
                new_combos.push(concat_combo);
            }

            combos = new_combos;
        }

        results.extend(combos);

        results
    }

    fn generate_operator_combinations_part_one(slots: usize) -> Vec<Vec<Operators>> {
        if slots == 0 {
            return vec![vec![]];
        }
        let mut combos = vec![vec![Operators::Add], vec![Operators::Multiply]];

        let mut results = Vec::new();

        for _ in 1..slots {
            let mut new_combos = Vec::new();

            for c in &combos {
                let mut add_combo = c.clone();
                add_combo.push(Operators::Add);
                new_combos.push(add_combo);

                let mut mul_combo = c.clone();
                mul_combo.push(Operators::Multiply);
                new_combos.push(mul_combo);
            }

            combos = new_combos;
        }

        results.extend(combos);

        results
    }

    fn parse_input(input: &str) -> Vec<Equation> {
        input
            .lines()
            .map(|line| Equation::new_from_input(line))
            .collect()
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
