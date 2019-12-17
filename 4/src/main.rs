fn is_valid_password(password: &String) -> bool {
    let mut index = 0;
    let mut has_double = false;
    while index + 2 <= password.len() {
        let split = &password[index..index + 2];
        let first = split.chars().nth(0).unwrap().to_digit(10);
        let second = split.chars().nth(1).unwrap().to_digit(10);

        if first == second {
            has_double = true;
        } else if first > second {
            return false;
        }
        index += 1;
    }
    has_double
}

fn part_1() {
    assert_eq!(is_valid_password(&String::from("111111")), true);
    assert_eq!(is_valid_password(&String::from("223450")), false);
    assert_eq!(is_valid_password(&String::from("123789")), false);
    let mut counter = 0;
    for i in 272091..815432 {
        if is_valid_password(&i.to_string()) {
            counter += 1
        }
    }
    println!("Part 1 result: {}", counter);
}

fn main() {
    part_1()
}
