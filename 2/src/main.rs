fn opcode_parse(opcodes: Vec<i32>) -> Vec<i32> {
    for opcode in opcodes.chunks(4) {
        let code = &opcode[0];
        match code {
            1 => {
                let params = &opcode[1..4];
                println!("should add, params: {:?}", params);
            }
            2 => {
                let params = &opcode[1..4];
                println!("should multi, params {:?}", params);
            }
            99 => {
                println!("EXIT PLEASE");
                break;
            }
            _ => println!("Unknown opcode: {}", code),
        }
    }
    return opcodes;
}

fn main() {
    let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

    let test = opcode_parse(input);
    println!("test: {:?}", test);
}
