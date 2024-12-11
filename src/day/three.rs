#[derive(Debug)]
enum Opcode {
    Mul(i32, i32),
    Do,
    Dont,
}

pub fn first(input: &str) -> usize {
    let opcodes = get_opcodes(input);

    let mut result = 0;

    for opcode in opcodes {
        if let Opcode::Mul(a, b) = opcode {
            result += a * b;
        }
    }

    result as usize
}

pub fn second(input: &str) -> usize {
    let mut do_multiplication = true;
    let opcodes = get_opcodes(input);

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
fn test_first() {
    assert_eq!(first(include_str!("../../test/fixture/three_first.txt")), 161);
}

#[test]
fn test_second() {
    assert_eq!(second(include_str!("../../test/fixture/three_second.txt")), 48);
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
