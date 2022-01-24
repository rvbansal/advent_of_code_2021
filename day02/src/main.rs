enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

use Direction::*;

fn parse_direction(line: &str) -> Direction {
    let split_line = line.split(' ').collect::<Vec<_>>();
    let amount = split_line[1].parse::<i32>().unwrap();
    match split_line[0] {
        "forward" => Forward(amount),
        "down" => Down(amount),
        "up" => Up(amount),
        _ => panic!("Invalid direction!"),
    }
}

fn get_product_of_directions(input: &str) -> i32 {
    let (horizonal, depth) =
        input
            .split('\n')
            .map(parse_direction)
            .fold((0, 0), |(h, d), direction| match direction {
                Forward(amount) => (h + amount, d),
                Down(amount) => (h, d + amount),
                Up(amount) => (h, d - amount),
            });

    horizonal * depth
}

fn get_product_of_directions_aim(input: &str) -> i32 {
    let (horizontal, depth, _aim) =
        input
            .split('\n')
            .map(parse_direction)
            .fold((0, 0, 0), |(h, d, a), direction| match direction {
                Forward(amount) => (h + amount, d + a * amount, a),
                Down(amount) => (h, d, a + amount),
                Up(amount) => (h, d, a - amount),
            });

    horizontal * depth
}

fn main() {
    println!(
        "Product of directions: {}",
        get_product_of_directions(include_str!("input.txt"))
    );
    println!(
        "Product of directions with aim: {}",
        get_product_of_directions_aim(include_str!("input.txt"))
    );
}
