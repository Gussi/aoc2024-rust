pub mod part {

    enum Opcode {
        Mul(i32, i32),
    }

    pub fn one(input: &str) -> i32 {
        let opcodes = get_mul_opcodes(input);

        let mut result = 0;

        for opcode in opcodes {
            match opcode {
                Opcode::Mul(a, b) => {
                    result += a * b;
                }
            }
        }

        result
    }

    #[test]
    fn one_test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(one(input), 161);
    }

    pub fn two(_input: &str) -> i32 {
        0
    }

    fn get_mul_opcodes(input: &str) -> Vec<Opcode> {
        let mut opcodes = Vec::new();

        let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        for cap in re.captures_iter(input) {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            opcodes.push(Opcode::Mul(a, b));
        }
        
        opcodes
    }
}
