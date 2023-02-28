use std::str::FromStr;

#[derive(Debug)]
#[derive(Clone)]
enum Operation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

/// Allow parsing of the operation enum from the input strings
impl FromStr for Operation {
    type Err = ();

    fn from_str(input: &str) -> Result<Operation, Self::Err> {
        match input {
            "+" => Ok(Operation::ADD),
            "-" => Ok(Operation::SUBTRACT),
            "*" => Ok(Operation::MULTIPLY),
            "/" => Ok(Operation::DIVIDE),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Monkey {
    id: u8,
    items: Vec<u64>,
    operation: Operation,
    operation_value: u64,
    test_div: u64,
    true_throw: u64,
    false_throw: u64,
}

/// Get id from line (1) of input blob
fn get_id(line: &str) -> u8 {
    line[7..line.len() - 1].parse::<u8>().unwrap()
}

/// Get items from line (2) of input blob
fn get_items(line: &str) -> Vec<u64> {
    line[18..line.len()]
        .split(", ")
        .into_iter()
        .map(|str_val| str_val.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .to_vec()
}

/// Get value from char n to end of input blob, as a u64
fn get_value_n_from_end(line: &str, n: usize) -> u64 {
    let section = &line[n..line.len()];

    let value = section.parse::<u64>();

    if value.is_ok() {
        return value.unwrap()
    } else {
        0  // 0 represents 'old' (old * old) in the operation application (should introduct new type tbh)
    }

    
}

/// Get value from char n to end of input blob, as a u64
fn get_operation(line: &str) -> Operation {
    Operation::from_str(&line.chars().nth(23).unwrap().to_string()).unwrap()
} 

/// Allow parsing of the direction enum from the input strings
impl FromStr for Monkey {
    type Err = ();

    fn from_str(blob: &str) -> Result<Monkey, Self::Err> {
        // Monkey 0:
        //   Starting items: 79, 98
        //   Operation: new = old * 19
        //   Test: divisible by 23
        //     If true: throw to monkey 2
        //     If false: throw to monkey 3

        let lines = blob.lines().collect::<Vec<&str>>();

        Ok(Monkey {
            id: get_id(lines[0]),
            items: get_items(lines[1]),
            operation: get_operation(lines[2]),
            operation_value: get_value_n_from_end(lines[2], 25), 
            test_div: get_value_n_from_end(lines[3], 21),
            true_throw: get_value_n_from_end(lines[4], 29),
            false_throw: get_value_n_from_end(lines[5], 30),
        })
    }
}

fn main() {
    let mut monkeys = include_str!("day11.txt")
        .split("\n\n").map(|blob| {
            Monkey::from_str(blob).unwrap()
        })
        .collect::<Vec<Monkey>>();
    

    let mut inspections: Vec<u64> = vec![0; monkeys.len() as usize];

    let to_mod = monkeys.iter().map(|m| {m.test_div}).product::<u64>();

    for _ in 0..10000 {  // rounds (part 2)

        for i in 0..monkeys.len() {

            for item in monkeys[i].items.clone() {

                // item = item % to_mod;

                // Monkey inspects item
                inspections[i] += 1;
                // Apply operation
                let mut op_value = monkeys[i].operation_value;
                if op_value == 0 {  // If operation_value is 0, that's an encoding for 'old'
                    op_value = item;
                }
                
                let new_value = match monkeys[i].operation {
                    Operation::ADD => (item + op_value) % to_mod,
                    Operation::SUBTRACT => (item - op_value) % to_mod,
                    Operation::MULTIPLY => (item * op_value) % to_mod,
                    Operation::DIVIDE => (item / op_value) % to_mod,
                };

                let current_monkey = monkeys[i].clone();
                if new_value % current_monkey.test_div == 0 {
                    monkeys[current_monkey.true_throw as usize].items.push(new_value);
                } else {
                    monkeys[current_monkey.false_throw as usize].items.push(new_value);
                }

            }

            monkeys[i].items = vec![];

        }

        // println!("{:#?}", monkeys[i])

    }

    inspections.sort();
    inspections.reverse();

    println!("{:?}", inspections);

    let worry = inspections[0] * inspections[1];

    println!("{:?}", worry);


}
