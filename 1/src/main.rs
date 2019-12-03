mod input_1;
mod input_2;

fn calculate_fuel(mass: i32) -> i32 {
    (mass as f64 / 3.0) as i32 - 2
}

fn calculate_fuel_with_fuel(fuel_mass: i32, sum: i32) -> i32 {
    let fuel_for_fuel = (fuel_mass as f64 / 3.0) as i32 - 2;
    if fuel_for_fuel < 0 {
        sum
    } else {
        calculate_fuel_with_fuel(fuel_for_fuel, sum + fuel_for_fuel)
    }
}

fn part_1() {
    println!("=== PART 1 ===");

    assert_eq!(calculate_fuel(12), 2);
    assert_eq!(calculate_fuel(14), 2);
    assert_eq!(calculate_fuel(1969), 654);
    assert_eq!(calculate_fuel(100756), 33583);

    let mut fuel_sum: i32 = 0;
    for i in input_1::INPUT.iter() {
        fuel_sum += calculate_fuel(*i);
    }

    println!("Total fual: {}", fuel_sum);
}

fn part_2() {
    println!("=== PART 2 ===");

    assert_eq!(calculate_fuel_with_fuel(14, 0), 2);
    assert_eq!(calculate_fuel_with_fuel(1969, 0), 966);
    assert_eq!(calculate_fuel_with_fuel(100756, 0), 50346);

    let mut fuel_sum: i32 = 0;
    for i in input_2::INPUT.iter() {
        fuel_sum += calculate_fuel_with_fuel(*i, 0);
    }
    println!("Total fuel: {}", fuel_sum);
}

fn main() {
    part_1();
    part_2();
}
