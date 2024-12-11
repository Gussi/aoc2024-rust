pub mod part {
    type SafetyRule = (i32, i32);
    type SafetyUpdate = Vec<i32>;

    struct SafetyManual {
        rules: Vec<SafetyRule>,
        updates: Vec<SafetyUpdate>,
    }

    impl SafetyManual {
        fn new() -> SafetyManual {
            SafetyManual {
                rules: Vec::new(),
                updates: Vec::new(),
            }
        }

        fn solve_one(&self) -> i32 {
            let mut answer = 0;

            for update in &self.updates {
                if self.is_valid(update) {
                    answer += update[update.len() / 2];
                }
            }

            answer
        }

        fn solve_two(&self) -> i32 {
            let mut answer = 0;

            for update in &self.updates {
                if !self.is_valid(update) {
                    let fixed_update = self.fix_update(update);
                    answer += fixed_update[fixed_update.len() / 2];
                }
            }

            answer
        }

        fn fix_update(&self, update: &SafetyUpdate) -> SafetyUpdate {
            let mut fixed_update = update.clone();

            for (i, &a) in update.iter().enumerate() {
                for (j, &b) in update.iter().enumerate().skip(i+1) {
                    if !self.rules.contains(&(a, b)) {
                        fixed_update[i] = b;
                        fixed_update[j] = a;

                        return self.fix_update(&fixed_update);
                    }
                }
            }

            fixed_update
        }

        // take pointer to specific update
        fn is_valid(&self, update: &SafetyUpdate) -> bool {
            for (i, &a) in update.iter().enumerate() {
                for (_, &b) in update.iter().enumerate().skip(i+1) {
                    if !self.rules.contains(&(a, b)) {
                        return false;
                    }
                }
            }

            true
        }

        fn new_from_input(input: &str) -> SafetyManual {
            let mut manual = SafetyManual::new();

            enum ParseState {
                Rules,
                Updates,
            }

            let mut state = ParseState::Rules;

            for line in input.lines() {

                if line.is_empty() {
                    state = ParseState::Updates;
                    continue;
                }

                match state {
                    ParseState::Rules => {
                        let rule = line
                            .split('|')
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();

                        manual.rules.push((rule[0], rule[1]));
                    }
                    ParseState::Updates => {
                        let update = line
                            .split(',')
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();

                        manual.updates.push(update);
                    }
                }
            }

            manual
        }
    }

    pub fn one(input: &str) -> usize {
        let manual = SafetyManual::new_from_input(input);
        manual.solve_one() as usize
    }

    pub fn two(_input: &str) -> usize {
        let manual = SafetyManual::new_from_input(_input);
        manual.solve_two() as usize
    }


    #[cfg(test)]
    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_one() {
        assert_eq!(one(TEST_INPUT), 143);
    }

    #[test]
    fn test_two() {
        assert_eq!(two(TEST_INPUT), 123);
    }
}
