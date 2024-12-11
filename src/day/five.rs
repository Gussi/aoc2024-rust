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

pub fn first(input: &str) -> usize {
    let manual = SafetyManual::new_from_input(input);
    manual.solve_one() as usize
}

pub fn second(_input: &str) -> usize {
    let manual = SafetyManual::new_from_input(_input);
    manual.solve_two() as usize
}


#[cfg(test)]
const TEST_INPUT: &str = include_str!("../../test/fixture/five.txt");

#[test]
fn test_first() {
    assert_eq!(first(TEST_INPUT), 143);
}

#[test]
fn test_second() {
    assert_eq!(second(TEST_INPUT), 123);
}
