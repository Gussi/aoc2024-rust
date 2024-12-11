pub mod part {

    #[derive(Debug)]
    enum Opcode {
        Mul(i32, i32),
        Do,
        Dont,
    }

    pub fn one(input: &str) -> usize {
        let opcodes = get_opcodes(input);

        let mut result = 0;

        for opcode in opcodes {
            if let Opcode::Mul(a, b) = opcode {
                result += a * b;
            }
        }

        result as usize
    }

    #[test]
    fn one_test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(one(input), 161);
    }

    pub fn two(_input: &str) -> usize {
        let mut do_multiplication = true;
        let opcodes = get_opcodes(_input);

        let mut result = 0;

        for opcode in opcodes {
            match opcode {
                Opcode::Mul(a, b) => {
                    if do_multiplication {
                        result += a * b;
                    }
                }
                Opcode::Do => {
                    do_multiplication = true;
                }
                Opcode::Dont => {
                    do_multiplication = false;
                }
            }
        }

        result as usize
    }

    #[test]
    fn two_test() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(two(input), 48);
    }

    fn get_opcodes(input: &str) -> Vec<Opcode> {
        let mut opcodes = Vec::new();

        let re = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
        for cap in re.captures_iter(input) {
            match &cap[0] {
                "do()" => {
                    opcodes.push(Opcode::Do);
                }
                "don't()" => {
                    opcodes.push(Opcode::Dont);
                }
                _ => {
                    let a = cap[1].parse::<i32>().unwrap();
                    let b = cap[2].parse::<i32>().unwrap();
                    opcodes.push(Opcode::Mul(a, b));
                }
            }
        }
        
        opcodes
    }
}
