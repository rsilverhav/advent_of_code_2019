struct OpcodeParams {
    src_index_1: usize,
    src_index_2: usize,
    destination_index: usize,
}

fn get_opcode_params(data: &[i32]) -> OpcodeParams {
    OpcodeParams {
        src_index_1: data[0] as usize,
        src_index_2: data[1] as usize,
        destination_index: data[2] as usize,
    }
}

fn opcode_parse(mut memory: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    while i < memory.len() {
        let instruction = memory[i];
        match instruction {
            1 => {
                let params = get_opcode_params(&memory[i + 1..i + 4]);
                memory[params.destination_index] =
                    memory[params.src_index_1] + memory[params.src_index_2];
                i += 3
            }
            2 => {
                let params = get_opcode_params(&memory[i + 1..i + 4]);
                memory[params.destination_index] =
                    memory[params.src_index_1] * memory[params.src_index_2];
                i += 3
            }
            99 => {
                break;
            }
            _ => i += 1,
        }
    }

    return memory;
}

fn get_memory() -> Vec<i32> {
    vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 19, 9, 23, 1, 5, 23, 27,
        1, 27, 9, 31, 1, 6, 31, 35, 2, 35, 9, 39, 1, 39, 6, 43, 2, 9, 43, 47, 1, 47, 6, 51, 2, 51,
        9, 55, 1, 5, 55, 59, 2, 59, 6, 63, 1, 9, 63, 67, 1, 67, 10, 71, 1, 71, 13, 75, 2, 13, 75,
        79, 1, 6, 79, 83, 2, 9, 83, 87, 1, 87, 6, 91, 2, 10, 91, 95, 2, 13, 95, 99, 1, 9, 99, 103,
        1, 5, 103, 107, 2, 9, 107, 111, 1, 111, 5, 115, 1, 115, 5, 119, 1, 10, 119, 123, 1, 13,
        123, 127, 1, 2, 127, 131, 1, 131, 13, 0, 99, 2, 14, 0, 0,
    ]
}

fn part_1() -> i32 {
    let mut memory = get_memory();

    memory[1] = 12;
    memory[2] = 2;
    opcode_parse(memory)[0]
}

fn part_2() -> i32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut memory = get_memory();
            memory[1] = noun;
            memory[2] = verb;
            let result = opcode_parse(memory)[0];
            if result == 19690720 {
                return noun * 100 + verb;
            }
        }
    }
    0
}

fn main() {
    assert_eq!(
        &opcode_parse(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50])[..],
        &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );

    assert_eq!(&opcode_parse(vec![1, 0, 0, 0, 99])[..], &[2, 0, 0, 0, 99]);

    assert_eq!(&opcode_parse(vec![2, 3, 0, 3, 99])[..], &[2, 3, 0, 6, 99]);

    assert_eq!(
        &opcode_parse(vec![2, 4, 4, 5, 99, 0])[..],
        &[2, 4, 4, 5, 99, 9801]
    );

    assert_eq!(
        &opcode_parse(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])[..],
        &[30, 1, 1, 4, 2, 5, 6, 0, 99]
    );

    println!("Part 1 result: {}", part_1());
    println!("Part 2 result: {}", part_2());
}
