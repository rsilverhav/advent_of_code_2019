fn opcode_parse(mut data: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    while i < data.len() {
        let opcode = data[i];
        match opcode {
            1 => {
                let val_a_index = data[i + 1] as usize;
                let val_b_index = data[i + 2] as usize;
                let to = data[i + 3] as usize;
                data[to] = data[val_a_index] + data[val_b_index];
                i += 3
            }
            2 => {
                let val_a_index = data[i + 1] as usize;
                let val_b_index = data[i + 2] as usize;
                let to = data[i + 3] as usize;
                data[to] = data[val_a_index] * data[val_b_index];
                i += 3
            }
            99 => {
                break;
            }
            _ => i += 1,
        }
    }

    return data;
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
}
